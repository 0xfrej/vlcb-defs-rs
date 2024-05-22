#!/bin/bash
set -f
set -u

OUTPUT=definition_new.yaml

cat << EOF > "$OUTPUT"
version: 1.1.0-dev
spec:
EOF

mapfile -t enums < <(cut -f1 -d , "defs.csv" | grep -v comment | sort | uniq)

for enum in "${enums[@]}"; do
    if [ -z "$enum" ]; then continue; fi
    echo "           $enum"
    enumComment=""
    enumContent=""
    enumAppend=""
    enum_head_finished=false

    while IFS="," read -r type name value comment
    do
      if [ "$type" = "$enum" ]; then
        value=${value//0X/0x}

        if [ -z "$name" ]; then
          if [ $enum_head_finished = false ]; then
            if [ -z "$comment" ]; then
                continue;
            fi
            enumComment="${enumComment}\n      ${value}${comment}"
          fi
        else
          enum_head_finished=true
          if [ -n "$comment" ]; then
            comment="${comment}"
          fi
          name=${name#OPC_}
          name=${name#MTYP_}
          name=${name#SASP_}
          name=${name#PB_}
          name=${name#CDAT_}
          name=${name#CMDERR_}
          name=${name#ERR_}
          name=${name#MANU_}
          name=${name#PAR_}
          name=${name#SSTAT_}
          name=${name#TMOD_}
          name=${name#CPUM_}
          name=${name#PF_}
          name=${name#SERVICE_}
          name=${name#MODE_}
          name=${name#GRSP_}
          name=${name#ID_}
          enumContent="${enumContent}\n      - identifier: ${name}\n        value: ${value}\n        comments: ${comment}"
        fi
      fi
    done < "defs.csv"

    enumComment=$(echo -e "${enumComment}")
    enumContent=$(echo -e "${enumContent}")
    enumAppend=$(echo -e "${enumAppend}")
    cat << EOF >> "$OUTPUT"
  - type: Enum
    identifier: $enum
    data_type: u8
    comments: |$enumComment
    body:$enumContent
EOF

done