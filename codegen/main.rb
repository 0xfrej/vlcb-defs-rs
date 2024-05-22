require_relative 'schema'

file_path = './definition.yaml'
begin
    validated_data = load_and_validate_yaml(file_path)
    puts "Validation passed"
    puts validated_data
rescue ArgumentError => e
    puts "Validation failed: #{e.message}"
end