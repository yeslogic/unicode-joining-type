UCD:=16.0.0

tables:
	yeslogic-ucd-generate joining-group --rust-enum ../ucd-generate/ucd-$(UCD) > src/joining_group_tables.rs
	yeslogic-ucd-generate joining-type --rust-enum ../ucd-generate/ucd-$(UCD) > src/joining_type_tables.rs
	cargo fmt


.PHONY: tables

