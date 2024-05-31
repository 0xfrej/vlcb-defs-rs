require_relative 'schema'
require 'iguvium'
require 'yaml'

def read_pdf()
  file = "Opcode Specification.pdf"

  pages = Iguvium.read(file)
  table_lines = []

  # loop over all pages and merge the data together
  # check if the page ranges are okay before running this !!
  #page num in pdf - 1
  (35..179).each do |i|
    table_lines += pages[i].extract_tables!.first.to_a
  end

  table_lines
end

def parse_pdf_lines(lines)
  result = []
  current_hash = nil

  lines.each do |entry|
    entry[0] = "Value" if entry[0].empty? && entry[1] == "Decimal" # Replace empty first element with "Value"

    if entry[0] == "Name"
      result << current_hash if current_hash # Append the previous hash to the result if it exists
      current_hash = {}
    end

    current_hash[entry[0] + entry[1]] = entry[2] unless current_hash.nil?
  end

  result << current_hash if current_hash # Append the last hash

  result
end

def transform_to_identifier(str)
  cleaned_str = str.gsub(/[^a-zA-Z\s]/, '').gsub('.', '')
  words = cleaned_str.split
  capitalized_words = words.map(&:capitalize)
  identifier = capitalized_words.join

  identifier
end

def transform_to_opcodes(opcdefs)
  opcodes = []

  opcdefs.each do |entry|
    opcode = {}

    opcode['identifier'] = transform_to_identifier(entry["Description"].split("\n").first)
    opcode['comments'] = entry['Description'] + "\n" + entry['Comment']
    if entry['ValueHex']
      opcode['value'] = ('0x' + entry['ValueHex']).to_i(16)
    elsif entry['ValueDecimal']
      opcode['value'] = entry['ValueDecimal'].to_i(10)
    end
    opcode['commentsFrom'] = 'docs/opcode/' + entry['Name'].downcase + '.md'

    comments = "# Opcode: #{entry['Name']}\n# Priority: #{entry['Priority']}\n# Services: #{entry['Services']}\n# Parameters: #{entry['Parameters']}\n# Conditions: #{entry['Conditions']}\n# Direction: #{entry['Direction']}\n# States / Modes: #{entry['States / Modes']}\n# Result: #{entry['Result']}"
    File.open("../"+opcode['commentsFrom'], 'w') do |file|
      file.write(comments)
    end
    opcodes << opcode      file.write(comments)opcode

  end

  file_path = "./opcodes.yaml"
  yml = YAML.load_file(file_path)
  yml['body'] = opcodes

  File.open(file_path, 'w') do |file|
    file.write(yml.to_yaml(line_width: -1).gsub!(/: (\d+)$/, ': 0x\1'))
  end
end

transform_to_opcodes(parse_pdf_lines(read_pdf()))