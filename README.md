A simple rust implementation of the bincode specification, based on serde

# Warning
This implementation is not completely accurate to the bincode specification, as it allows dictionaries to not be sorted by their field-this can be worked around with [https://serde.rs/field-attrs.html](serde's file attributes)