use dialoguer::{theme::ColorfulTheme, Input};
use comfy_table::*;
use comfy_table::presets::UTF8_FULL;

fn main() {
    let config = LoanConfig::from_prompt_wizard();
    let loan = config.compute_loan();

    let (witdh, _) = term_size::dimensions().unwrap_or((80, 80));
    let mut table = Table::new();
    table.load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_width(witdh as u16)
        .set_header(vec![
            Cell::new("Month").add_attribute(Attribute::Bold),
            Cell::new("Starting principal").fg(Color::Blue),
            Cell::new("Payment amount").fg(Color::Magenta),
            Cell::new("Principal payment").fg(Color::Blue),
            Cell::new("Interests payment").fg(Color::Red),
            Cell::new("Remaining principal").fg(Color::Blue),
        ]);

    for payment in loan.payments {
        table.add_row(vec![
            Cell::new(payment.month_index + 1).add_attribute(Attribute::Bold),
            Cell::new(format!("{:.02}{}", payment.start_principal, loan.config.currency)).fg(Color::Blue),
            Cell::new(format!("{:.02}{}", payment.value, loan.config.currency)).fg(Color::Magenta),
            Cell::new(format!("{:.02}{}", payment.pricipal_part, loan.config.currency)).fg(Color::Blue),
            Cell::new(format!("{:.02}{}", payment.interests_part, loan.config.currency)).fg(Color::Red),
            Cell::new(format!("{:.02}{}", payment.end_principal, loan.config.currency)).fg(Color::Blue),
        ]);
    }

    println!("{table}");
    println!("You will pay a total of {:.2}{} in interests.", loan.total_paid_interests, loan.config.currency);
}

struct LoanConfig {
    currency: String,
    principal: f64,
    annual_rate: f64,
    duration_in_months: usize,
}

struct LoanPayment {
    month_index: usize,
    start_principal: f64,
    end_principal: f64,
    interests_part: f64,
    pricipal_part: f64,
    value: f64,
}

struct Loan {
    config: LoanConfig,
    payments: Vec<LoanPayment>,
    total_paid_interests: f64,
}

impl LoanConfig {
    fn from_prompt_wizard() -> Self {
        let currency: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Your currency")
            .interact_text()
            .unwrap();

        let principal: f64 = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Loan amount")
            .interact_text()
            .unwrap();

        let annual_rate: f64 = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Annual interest rates (%)")
            .interact_text()
            .map(|r: f64| r / 100.)
            .unwrap();

        let duration_in_months: usize = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Duration in months")
            .interact_text()
            .unwrap();

        Self {
            currency,
            principal,
            annual_rate,
            duration_in_months,
        }
    }

    fn compute_monthly_payments(&self) -> f64 {
        let i = self.annual_rate / 12.;
        let p = self.principal;
        let n = self.duration_in_months as f64;

        p * (i * (1. + i).powf(n) / ((1. + i).powf(n) - 1.))
    }

    fn compute_payments(&self) -> Vec<LoanPayment> {
        let monthly_payments = self.compute_monthly_payments();

        let mut payments = Vec::with_capacity(self.duration_in_months);
        let mut principal = self.principal;

        for month_index in 0..self.duration_in_months {
            let start_principal = principal;
            let interests_part = self.annual_rate / 12. * principal;
            let pricipal_part = monthly_payments - interests_part;
            let end_principal = principal - pricipal_part;

            payments.push(LoanPayment {
                month_index,
                start_principal,
                end_principal,
                interests_part,
                pricipal_part,
                value: pricipal_part + interests_part
            });

            principal = end_principal;
        }

        payments
    }

    fn compute_loan(self) -> Loan {
        let payments = self.compute_payments();
        let total_paid_interests = payments.iter().fold(0., |i, payment| i + payment.interests_part);

        Loan {
            config: self,
            payments,
            total_paid_interests,
        }
    }
}
