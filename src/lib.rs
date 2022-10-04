use miette::Diagnostic;
use thiserror::Error;

pub struct LoanBuilder {
    pub principal: f64,
    pub annual_rate: f64,
    pub duration_in_months: usize,
}

impl Default for LoanBuilder {
    fn default() -> Self {
        Self {
            principal: 0.,
            annual_rate: 0.,
            duration_in_months: 1,
        }
    }
}

#[derive(Error, Diagnostic, Debug)]
pub enum LoanBuilderError {
    #[error("Loan principal must be a positive number, received {0}")]
    InvalidPrincipal(f64),

    #[error("Loan annual rate must be a number between 0 and 1, received {0}")]
    InvalidAnnualRate(f64),

    #[error("Loan duration must be in month and greater than  0, received {0}")]
    InvalidDuration(usize),
}

impl LoanBuilder {
    pub fn with_principal(mut self, principal: f64) -> Result<Self, LoanBuilderError> {
        if principal < 0. {
            return Err(LoanBuilderError::InvalidPrincipal(principal));
        }

        self.principal = principal;
        Ok(self)
    }

    pub fn with_annual_rate(mut self, annual_rate: f64) -> Result<Self, LoanBuilderError> {
        if !(0. ..=1.).contains(&annual_rate) {
            return Err(LoanBuilderError::InvalidAnnualRate(annual_rate));
        }

        self.annual_rate = annual_rate;
        Ok(self)
    }

    pub fn with_duration_in_months(
        mut self,
        duration_in_months: usize,
    ) -> Result<Self, LoanBuilderError> {
        if duration_in_months < 1 {
            return Err(LoanBuilderError::InvalidDuration(duration_in_months));
        }

        self.duration_in_months = duration_in_months;
        Ok(self)
    }

    pub fn build(self) -> Result<Loan, LoanBuilderError> {
        let LoanBuilder {
            principal,
            annual_rate,
            duration_in_months,
        } = self;

        let payments = {
            let mut payments = Vec::with_capacity(duration_in_months);
            let mut remaining_principal = principal;

            let monthly_payment_amount = {
                let i = annual_rate / 12.;
                let p = principal;
                let n = duration_in_months as f64;

                p * (i * (1. + i).powf(n) / ((1. + i).powf(n) - 1.))
            };

            for month_index in 0..duration_in_months {
                let start_principal = remaining_principal;
                let interests_part = annual_rate / 12. * remaining_principal;
                let principal_part = monthly_payment_amount - interests_part;
                let end_principal = remaining_principal - principal_part;

                payments.push(Payment {
                    month_index,
                    start_principal,
                    interests_part,
                    principal_part,
                    amount: principal_part + interests_part,
                });

                remaining_principal = end_principal;
            }

            payments
        };

        let total_paid_interests = payments
            .iter()
            .fold(0., |i, payment| i + payment.interests_part);

        Ok(Loan {
            principal,
            annual_rate,
            duration_in_months,
            payments,
            total_paid_interests,
        })
    }
}

pub struct Payment {
    pub month_index: usize,
    pub start_principal: f64,
    pub interests_part: f64,
    pub principal_part: f64,
    pub amount: f64,
}

pub struct Loan {
    pub principal: f64,
    pub annual_rate: f64,
    pub duration_in_months: usize,
    pub payments: Vec<Payment>,
    pub total_paid_interests: f64,
}

impl Loan {
    pub fn builder() -> LoanBuilder {
        LoanBuilder::default()
    }
}
