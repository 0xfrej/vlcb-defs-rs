.PHONY: generate
generate: clean
	bundle exec ruby ${CURDIR}/libgen.rb

.PHONY: clean
clean:
	rm -f ${ROOT_DIR}/src/generated.rs

#todo : install ghostscript if missing
.PHONY: install
install:
	bundle install