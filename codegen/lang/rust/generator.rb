require_relative "../../template"
require 'tomlib'
require 'set'

TEMPLATE_PATH = File.expand_path(File.join(File.dirname(__FILE__), 'templates/imports.erb'))
CARGO_TOML_PATH = File.expand_path('../../../../Cargo.toml', __FILE__)
OUTPUT_PATH = File.expand_path(File.join(File.dirname(__FILE__), 'output'))
FORMATTER_PATH = File.expand_path(File.join(File.dirname(__FILE__), 'formatter'))
TARGET_PATH = File.expand_path(File.join(OUTPUT_PATH, 'generated.rs'))

def update_crate_version(spec)
  cargo_toml_content = File.read(CARGO_TOML_PATH)
  cargo_toml = Tomlib.load(cargo_toml_content)

  begin
    current_version_str = cargo_toml['package']['version']
    new_version_str = spec[:version]
    current_version = Semverse::Version.new(current_version_str)
    new_version = Semverse::Version.new(new_version_str)
    if current_version == new_version
      puts "Current Cargo.toml has the same version as the code generation spec. Please be sure that you're not overwriting already published version"
    elsif current_version < new_version
      cargo_toml['package']['version'] = new_version_str

      File.write(CARGO_TOML_PATH, Tomlib.dump(cargo_toml, indent: false))

      puts "Updated from version #{current_version_str} to #{new_version_str} in Cargo.toml."
    else
      raise "New version #{new_version_str} is not a bump over the current version #{current_version_str}."
    end
  rescue Semverse::InvalidVersionFormat => e
    raise "Invalid version format: #{e.message}"
  end
end

def generate_rust(spec)
  FileUtils.mkdir_p(OUTPUT_PATH) unless Dir.exist?(OUTPUT_PATH)

  puts "Generating Rust files"
  update_generated_files(spec)
  puts "Updating Cargo library version to " + spec[:version]
  update_crate_version(spec)
  puts "Running formatter"
  format_files()
  puts "Done"
end

def format_files()
  %x[cd #{FORMATTER_PATH} && cargo run formatter]
end

def update_generated_files(spec)
  renderer = Renderer.new(File.join(File.dirname(__FILE__), 'templates'))
  ctx = { imports: Set.new, body: "" }

  for spec_item in spec[:spec]
    case spec_item[:type]
    when "Enum"
      ctx = gen_enum(ctx, renderer, spec_item)
    when "Flags"
      ctx = gen_flags(ctx, renderer, spec_item)
    else
      raise "Unimplemented codegen spec item type #{spec_item[:type]}"
    end
  end

  ctx = gen_imports(ctx, renderer)

  File.write(TARGET_PATH, ctx[:body])
  system('rustfmt', TARGET_PATH)
end

def gen_imports(ctx, renderer)
  output = renderer.r('imports', imports: ctx[:imports])
  ctx[:body] = "#{output}#{ctx[:body]}"
  return ctx
end

def gen_enum(ctx, renderer, enum)
  ctx[:imports].add('num_enum::IntoPrimitive')
  ctx[:imports].add('num_enum::TryFromPrimitive')
  ctx[:imports].add('num_enum::UnsafeFromPrimitive')

  extra_derives = Set.new

  enum[:comments] = parse_comments(enum[:comments])
  enum[:body].each do |variant|
    variant['comments'] = parse_comments(variant['comments'])
    if variant['is_default'] == true
      ctx[:imports].add('num_enum::FromPrimitive')
      extra_derives.add('FromPrimitive')
    end
  end

  enum[:annotations] = [
    "derive(Debug, Copy, Clone, TryFromPrimitive, UnsafeFromPrimitive, IntoPrimitive, Eq, PartialEq#{", " if extra_derives.empty?}#{extra_derives.join(', ')})",
    'cfg_attr(feature = "defmt", derive(defmt::Format))',
    "repr(#{enum[:data_type]})",
  ]

  output = renderer.r('enum', enum: enum)
  ctx[:body] = "#{output}#{ctx[:body]}"

  return ctx
end

def gen_flags(ctx, renderer, flags)
  ctx[:imports].add('bitflags::bitflags')

  flags[:data_type_size] = flags[:data_type][1..-1]

  flags[:comments] = parse_comments(flags[:comments])
  flags[:body].each do |flag|
    flag['comments'] = parse_comments(flag['comments'])
  end

  flags[:annotations] = [
    'derive(Debug, Copy, Clone)'
  ]

  output = renderer.r('flags', flags: flags)
  ctx[:body] = "#{output}#{ctx[:body]}"

  return ctx
end

def parse_comments(comments)
  return comments.is_a?(String) ? comments.split("\n") : []
end