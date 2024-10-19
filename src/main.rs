trait Client {
    fn accept(&self, visitor: &dyn Visitor);
}

struct Hospital {
    name: String,
}

impl Client for Hospital {
    fn accept(&self, visitor: &dyn Visitor) {
        visitor.visit_hospital(self);
    }
}

struct Bank {
    name: String,
}

impl Client for Bank {
    fn accept(&self, visitor: &dyn Visitor) {
        visitor.visit_bank(self);
    }
}

trait Visitor {
    fn visit_bank(&self, bank: &Bank);
    fn visit_hospital(&self, hospital: &Hospital);
}

struct EmailSender;

impl EmailSender {
    pub fn send(&self, client: &dyn Client) {
        client.accept(self);
    }
}

impl Visitor for EmailSender {
    fn visit_bank(&self, bank: &Bank) {
        println!("{:#?} is sending an email...", bank.name)
    }

    fn visit_hospital(&self, hospital: &Hospital) {
        println!("{:#?} is sending an email...", hospital.name)
    }
}

fn main() {
    let email_sender = EmailSender;

    let b = Bank {
        name: "b23".to_string(),
    };
    b.accept(&email_sender);

    let h = Hospital {
        name: "h88".to_string(),
    };
    h.accept(&email_sender);

    email_sender.send(&b);
}
