require 'yaml'
require 'dry-validation'
require 'dry-types'
require 'semverse'

module Types
    include Dry.Types()

    BoolWithDefaultFalse = Types::Bool.default(false)
end

module ValueCanBeIntegerOrChar
    def self.included(base)
        base.class_eval do
            rule(:value, :data_type) do
                if values[:data_type].start_with?('u')
                    key.failure('must be a positive integer') unless values[:value].is_a?(Integer) && values[:value] >= 0
                elsif values[:data_type].start_with?('i')
                    key.failure('must be an integer') unless values[:value].is_a?(Integer)
                elsif values[:data_type] == 'char'
                    key.failure('must be a single character') unless values[:value].is_a?(String) && values[:value].length == 1
                end
            end
        end
    end
end

class FlagsBodySchema < Dry::Validation::Contract
    params do
        required(:identifier).filled(Types::String)
        required(:value).filled(Types::Integer)
        optional(:comments).maybe(Types::String)
    end
end

class EnumBodySchema < Dry::Validation::Contract
    include ValueCanBeIntegerOrChar

    params do
        required(:identifier).filled(Types::String)
        required(:value).filled(Types::Any)
        optional(:is_default).maybe(Types::BoolWithDefaultFalse)
        optional(:comments).maybe(Types::String)
    end
end

class CodegenSpecSchema < Dry::Validation::Contract
    params do
        required(:type).filled(Types::String)
        optional(:data_type).maybe(Types::String)
        required(:identifier).filled(Types::String)
        optional(:comments).maybe(Types::String)
        optional(:body).array(:hash)
    end

    rule(:type) do
        unless %w[Enum Flags].include?(value)
        key.failure('must be either Enum or Flags')
        end
    end

    rule(:data_type, :type) do
    if %w[Enum Flags].include?(values[:type])
        key.failure('must be present') if values[:data_type].nil?
        if values[:data_type] && !%w[u8 u16 u32 u64 i8 i16 i32 i64 char].include?(values[:data_type])
            key.failure('must be one of: u8, u16, u32, u64, i8, i16, i32, i64, char')
        end
        end
    end

    rule(:body, :type) do
    next unless values[:body]

    body_schema = values[:type] == 'Enum' ? EnumBodySchema.new : FlagsBodySchema.new
    values[:body].each_with_index do |body_item, index|
        body_item[:data_type] = values[:data_type]
        result = body_schema.call(body_item)
        unless result.success?
            result.errors.to_h.each do |field, errors|
            key([:body, index, field]).failure(errors.join(', '))
            end
        end
        end
    end
end

class CodegenSchema < Dry::Validation::Contract
    params do
        required(:version).filled(Types::String)
        required(:spec).array(CodegenSpecSchema.schema)
    end

    rule(:version) do
        begin
            Semverse::Version.new(value)
        rescue Semverse::InvalidVersionFormat
            key.failure('must be a valid semver version')
        end
    end
end

def load_and_validate_yaml(file_path)
    data = YAML.load_file(file_path)
    schema = CodegenSchema.new
    validation_result = schema.call(data)

    return validation_result.to_h if validation_result.success?

    raise ArgumentError, "Validation failed: #{validation_result.errors.to_h}"
end