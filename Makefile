tables:
	yeslogic-ucd-generate joining-group --rust-enum ../ucd-generate/ucd-15.0.0 > src/joining_group_tables.rs
	yeslogic-ucd-generate joining-type --rust-enum ../ucd-generate/ucd-15.0.0 > src/joining_type_tables.rs

.PHONY: tables

