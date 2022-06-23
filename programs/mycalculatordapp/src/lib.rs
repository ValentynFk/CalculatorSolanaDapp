use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod mycalculatordapp {
    use super::*;
    pub fn create(ctx: Context<Create>, init_message: String) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.greeting = init_message;
        Ok(())
    }

    pub fn add(ctx: Context<Addition>, lhs: i64, rhs: i64) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = lhs + rhs;
        Ok(())
    }

    pub fn sub(ctx: Context<Subtraction>, lhs: i64, rhs: i64) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = lhs - rhs; // We always subtract right from left
        Ok(())
    }

    pub fn mul(ctx: Context<Multiplication>, lhs: i64, rhs: i64) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = lhs * rhs;
        Ok(())
    }

    pub fn div(ctx: Context<Division>, lhs: i64, rhs: i64) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        // TODO: check if lhs is bigger than rhs
        let res = lhs / rhs; // We always divide right by left
        calculator.result = res;
        calculator.remainder = lhs % res;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer=user, space=264)]
    pub calculator: Account<'info, Calculator>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct Addition<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

#[derive(Accounts)]
pub struct Subtraction<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

#[derive(Accounts)]
pub struct Multiplication<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

#[derive(Accounts)]
pub struct Division<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

/// Calculator account (contains necessary information that the calculator needs)
#[account]
pub struct Calculator {
    pub greeting: String,
    pub result: i64,
    pub remainder: i64,
}
