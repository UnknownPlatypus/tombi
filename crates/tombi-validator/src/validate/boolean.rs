use tombi_diagnostic::SetDiagnostics;
use tombi_document_tree::ValueImpl;
use tombi_future::{BoxFuture, Boxable};
use tombi_schema_store::{ValueSchema, ValueType};

use super::{validate_all_of, validate_any_of, validate_one_of, Validate};

impl Validate for tombi_document_tree::Boolean {
    fn validate<'a: 'b, 'b>(
        &'a self,
        accessors: &'a [tombi_schema_store::SchemaAccessor],
        current_schema: Option<&'a tombi_schema_store::CurrentSchema<'a>>,
        schema_context: &'a tombi_schema_store::SchemaContext,
    ) -> BoxFuture<'b, Result<(), Vec<tombi_diagnostic::Diagnostic>>> {
        async move {
            let mut diagnostics = vec![];

            if let Some(current_schema) = current_schema {
                match current_schema.value_schema.value_type().await {
                    ValueType::Boolean
                    | ValueType::OneOf(_)
                    | ValueType::AnyOf(_)
                    | ValueType::AllOf(_) => {}
                    ValueType::Null => return Ok(()),
                    value_schema => {
                        crate::Error {
                            kind: crate::ErrorKind::TypeMismatch {
                                expected: value_schema,
                                actual: self.value_type(),
                            },
                            range: self.range(),
                        }
                        .set_diagnostics(&mut diagnostics);

                        return Err(diagnostics);
                    }
                }
                let boolean_schema = match current_schema.value_schema.as_ref() {
                    ValueSchema::Boolean(boolean_schema) => boolean_schema,
                    ValueSchema::OneOf(one_of_schema) => {
                        return validate_one_of(
                            self,
                            accessors,
                            one_of_schema,
                            current_schema,
                            schema_context,
                        )
                        .await
                    }
                    ValueSchema::AnyOf(any_of_schema) => {
                        return validate_any_of(
                            self,
                            accessors,
                            any_of_schema,
                            current_schema,
                            schema_context,
                        )
                        .await
                    }
                    ValueSchema::AllOf(all_of_schema) => {
                        return validate_all_of(
                            self,
                            accessors,
                            all_of_schema,
                            current_schema,
                            schema_context,
                        )
                        .await
                    }
                    _ => unreachable!("Expected a Boolean schema"),
                };

                let value = self.value();
                if let Some(enumerate) = &boolean_schema.enumerate {
                    if !enumerate.contains(&value) {
                        crate::Error {
                            kind: crate::ErrorKind::Eunmerate {
                                expected: enumerate.iter().map(ToString::to_string).collect(),
                                actual: value.to_string(),
                            },
                            range: self.range(),
                        }
                        .set_diagnostics(&mut diagnostics);
                    }
                }
            }

            if diagnostics.is_empty() {
                Ok(())
            } else {
                Err(diagnostics)
            }
        }
        .boxed()
    }
}
