use anchor_lang::prelude::*;

declare_id!("2MM1n3msVUagVmivbvXGPKwxpJNtcvKKjMM1y9TZmiso");

#[program]
pub mod expense_tracker {
    use super::*;

    pub fn initializeExpense(
        ctx: Context<Initialize>,
        id: u64,
        merchant_name: String,
        amount: f64,
    ) -> Result<()> {
        let expense_account: &mut Account<ExpenseAccount> = &mut ctx.accounts.expense;
        expense_account.amount = amount;
        expense_account.merchant_name = merchant_name;
        expense_account.id = id;
        expense_account.owner = *ctx.accounts.authority.key;

        Ok(())
    }

    pub fn modify_expense(
        ctx: Context<ModifyExpense>,
        _id: u64,
        merchant_name: String,
        amount: f64,
    ) -> Result<()> {
        let expense_account: &mut Account<ExpenseAccount> = &mut ctx.accounts.expense_account;
        expense_account.merchant_name = merchant_name;
        expense_account.amount = amount;

        Ok(())
    }

    pub fn delete_expense(_ctx: Context<DeleteExpense>, _id: u64) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct Initialize<'info> {
    #[account(mut)]
    authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        space = 8 + 8 + 32 + (4 + 12) + 8 + 1,
        seeds = [b"expense", authority.key().as_ref(), id.to_le_bytes().as_ref()],
        bump
    )]
    expense: Account<'info, ExpenseAccount>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct ModifyExpense<'info> {
    #[account(mut)]
    authority: Signer<'info>,
    #[account(
        mut,
        seeds = [b"expense", authority.key().as_ref(), id.to_le_bytes().as_ref()],
        bump
    )]
    expense_account: Account<'info, ExpenseAccount>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct DeleteExpense<'info> {
    #[account(mut)]
    authority: Signer<'info>,
    #[account(
        mut,
        close = authority,
        seeds = [b"expense", authority.key().as_ref(), id.to_le_bytes().as_ref()], 
        bump
    )]
    pub expense_account: Account<'info, ExpenseAccount>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Default)]
pub struct ExpenseAccount {
    id: u64,
    merchant_name: String,
    amount: f64,
    owner: Pubkey,
}
