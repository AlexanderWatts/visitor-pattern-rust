pub trait Visitor {
    type T;

    fn visit_property(&self, property: &Property) -> Self::T;
}

pub trait AstNode {
    type T;
    fn accept(&self, visitor: &dyn Visitor<T = Self::T>) -> Self::T;
}

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&self, root: &dyn AstNode<T = String>) -> String {
       root.accept(self)
    }
}

impl Visitor for AstPrinter {
    type T = String;

    fn visit_property(&self, property: &Property) -> String {
        format!(
            "{}{}{}",
            property.identifier, property.colon, property.value
        )
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
    type T = String;

    fn accept(&self, visitor: &dyn Visitor<T = Self::T>) -> Self::T {
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

    #[test]
    fn invoke_via_visitor() {
        let property = Property::new("message", ":", "Hello, World!");

        let ast_printer = AstPrinter;
        let res = ast_printer.print(&property);
        
        assert_eq!(res, "message:Hello, World!");
    }
}
