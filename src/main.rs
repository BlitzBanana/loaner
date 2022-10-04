use comfy_table::presets::UTF8_FULL;
use comfy_table::*;
use dialoguer::{theme::ColorfulTheme, Input};
use loaner::Loan;
use miette::{IntoDiagnostic, Result, WrapErr};

fn main() -> Result<()> {
    let (loan, currency) = prompt_wizard()?;

    let (width, _) = term_size::dimensions().unwrap_or((80, 80));
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_width(width as u16)
        .set_header(vec![
            Cell::new("Month").add_attribute(Attribute::Bold),
            Cell::new("Starting principal")
                .add_attribute(Attribute::Bold)
                .fg(Color::Grey),
            Cell::new("Principal payment")
                .add_attribute(Attribute::Bold)
                .fg(Color::Blue),
            Cell::new("Interests payment")
                .add_attribute(Attribute::Bold)
                .fg(Color::Red),
            Cell::new("Payment amount")
                .add_attribute(Attribute::Bold)
                .fg(Color::Magenta),
        ]);

    for payment in loan.payments {
        table.add_row(vec![
            Cell::new(payment.month_index + 1).add_attribute(Attribute::Bold),
            Cell::new(format!("{:.02}{}", payment.start_principal, currency)).fg(Color::Grey),
            Cell::new(format!("{:.02}{}", payment.principal_part, currency)).fg(Color::Blue),
            Cell::new(format!("{:.02}{}", payment.interests_part, currency)).fg(Color::Red),
            Cell::new(format!("💰 {:.02}{}", payment.amount, currency))
                .add_attribute(Attribute::Bold)
                .fg(Color::Magenta),
        ]);
    }

    println!("{table}");
    println!(
        "You will pay a total of {:.2}{} in interests.",
        loan.total_paid_interests, currency
    );

    Ok(())
}

fn prompt_wizard() -> Result<(Loan, String)> {
    let currency: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Your currency")
        .interact_text()
        .into_diagnostic()
        .wrap_err("Unable to read `currency` from terminal input")?;

    let loan = Loan::builder()
        .with_principal(
            Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Loan amount")
                .interact_text()
                .into_diagnostic()
                .wrap_err("Unable to read `loan amount` from terminal input")?,
        )
        .into_diagnostic()
        .wrap_err("Invalid loan amount")?
        .with_annual_rate(
            Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Annual interest rates (%)")
                .interact_text()
                .map(|r: f64| r / 100.)
                .into_diagnostic()
                .wrap_err("Unable to read `annual rate` from terminal input")?,
        )
        .into_diagnostic()
        .wrap_err("Invalid loan annual interest rates")?
        .with_duration_in_months(
            Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Duration in months")
                .interact_text()
                .into_diagnostic()
                .wrap_err("Unable to read `duration` from terminal input")?,
        )
        .into_diagnostic()
        .wrap_err("Invalid loan duration")?
        .build()
        .into_diagnostic()
        .wrap_err("Invalid loan amount")?;

    Ok((loan, currency))
}
