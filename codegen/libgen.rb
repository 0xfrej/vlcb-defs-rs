require_relative 'schema'
require_relative 'lang/rust/generator'

file_path = './definition.yaml'
begin
    spec = load_and_validate_yaml(file_path)
    puts "Validation passed"
rescue ArgumentError => e
    raise "Validation failed: #{e.message}"
end

generate_rust(spec)