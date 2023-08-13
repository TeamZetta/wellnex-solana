use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct UserProfile {
    pub authority: Pubkey,
    pub last_prescription: u8,
    pub prescription_count: u8
}

#[account]
#[derive(Default)]
pub struct PrescriptionAccount {
    pub authority: Pubkey,
    pub idx: u8,
    pub name: String,
    pub height: String,
    pub weight: String,
    pub date: String,
    pub disease: String,
    pub medication: ,
    pub marked: bool,
}