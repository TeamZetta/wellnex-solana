ghluse anchor_lang::prelude::*;

pub mod constant;
pub mod states;
pub mod error;
use crate::{constant::*, error::*, states::*};

declare_id!("DWjTv1cqkrmxFxMha4ufbkTenCGdo3U9CpyihyxHKYHs");

#[program]
pub mod well_nex {
    use super::*;

    // Add user profile to the blockchain
    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
        // Initialize User with default data
        let user_profile = &mut ctx.accounts.user_profile;
        user_profile.authority = ctx.accounts.authority.key();
        user_profile.last_prescription = 0;
        user_profile.prescription_count = 0;

        Ok(())
    }

    // Add a Prescription
    pub fn add_prescription(ctx: Context<AddPrescription>, _name: String, _height: String, _weight: String, _disease: String, _date: String) -> Result<()> {
        // Fill Prescription account with proper values
        let prescription_account = &mut ctx.accounts.prescription_account;
        let user_profile = &mut ctx.accounts.user_profile;

        prescription_account.authority = ctx.accounts.authority.key();
        prescription_account.idx = user_profile.last_prescription;
        prescription_account.marked = false;
        prescription_account.name = _name;
        prescription_account.height = _height;
        prescription_account.weight = _weight;
        prescription_account.date = _date;
        prescription_account.disease = _disease;

        // Increase prescription idx
        user_profile.last_prescription = user_profile.last_prescription.checked_add(1).unwrap();

        // Increase total prescription count
        user_profile.prescription_count = user_profile.prescription_count.checked_add(1).unwrap();

        Ok(())
    }

    // Mark a Prescription
    pub fn mark_prescription(ctx: Context<MarkPrescription>, _prescription_idx: u8) -> Result<()> {
        // Change marked to TRUE
        let prescription_account = &mut ctx.accounts.prescription_account;
        require!(!prescription_account.marked, FetchError::AlreadyMarked);

        prescription_account.marked = true;

        Ok(())
    }

    
    // Delete Prescription
    pub fn delete_prescription(ctx: Context<RemovePrescription>, _prescription_idx: u8) -> Result<()> {
        // Decrement total Prescription count
        let user_profile = &mut ctx.accounts.user_profile;
        user_profile.prescription_count = user_profile.prescription_count
        .checked_sub(1)
        .unwrap();

        // No need to decrease last prescription idx
        // Prescription PDA already closed in context

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction()]
pub struct InitializeUser<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        seeds = [USER_TAG, authority.key().as_ref()],
        bump,
        payer = authority,
        space = 8 + std::mem::size_of::<UserProfile>(),
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction()]
pub struct AddPrescription<'info> {
    #[account(
        mut,
        seeds = [USER_TAG, authority.key().as_ref()],
        bump,
        has_one = authority,
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    #[account(
        init,
        seeds = [PRESCRIPTION_TAG, authority.key().as_ref(), &[user_profile.last_prescription as u8].as_ref()],
        bump,
        payer = authority,
        space = std::mem::size_of::<PrescriptionAccount>() + 8,
    )]
    pub prescription_account: Box<Account<'info, PrescriptionAccount>>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(prescription_idx: u8)]
pub struct MarkPrescription<'info> {
    #[account(
        mut,
        seeds = [USER_TAG, authority.key().as_ref()],
        bump,
        has_one = authority,
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    #[account(
        mut,
        seeds = [PRESCRIPTION_TAG, authority.key().as_ref(), &[prescription_idx].as_ref()],
        bump,
        has_one = authority,
    )]
    pub prescription_account: Box<Account<'info, PrescriptionAccount>>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
#[instruction(prescription_idx: u8)]
pub struct RemovePrescription<'info> {
    #[account(
        mut,
        seeds = [USER_TAG, authority.key().as_ref()],
        bump,
        has_one = authority,
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

   #[account(
       mut,
       close = authority,
       seeds = [PRESCRIPTION_TAG, authority.key().as_ref(), &[prescription_idx].as_ref()],
       bump,
       has_one = authority,
   )] 
     pub prescription_account: Box<Account<'info, PrescriptionAccount>>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}