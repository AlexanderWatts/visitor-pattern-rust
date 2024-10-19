pub trait Visitor<T> {
    fn visit_property(&self, property: &Property) -> T;
}
pub trait AstNode {
    fn accept<T>(&self, visitor: &dyn Visitor<T>) -> T; 
}

pub struct AstPrinter;

impl Visitor<String> for AstPrinter {
    fn visit_property(&self, property: &Property) -> String {
        format!("{}{}{}", property.identifier, property.colon, property.value)
    }
}

#[derive(Debug)]
pub struct Property {
    pub identifier: String,
    pub colon: String,
    pub value: String,
}

impl Property {
    pub fn new(identifier: &str, colon: &str, value: &str) -> Self {
        Self {
            identifier: String::from(identifier),
            colon: String::from(colon),
            value: String::from(value),
        }
    }
}

impl AstNode for Property {
    fn accept<T>(&self, visitor: &dyn Visitor<T>) -> T {
        visitor.visit_property(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_property() {
        let property = Property::new("message", ":", "Hello, World!");

        assert_eq!(property.identifier, "message");
        assert_eq!(property.colon, ":");
        assert_eq!(property.value, "Hello, World!");
    }

    #[test]
    fn create_ast_printer() {
        let ast_printer = AstPrinter;

        let property = Property::new("message", ":", "Hello, World!");
        let stringified = property.accept(&ast_printer);

        assert_eq!(stringified, "message:Hello, World!");
    }
}
