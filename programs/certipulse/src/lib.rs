use anchor_lang::prelude::*;

declare_id!("C4q98LErU614JFw48JEHsHwz3TVrJ2jBcx4CJeUxkyzn");

#[program]
pub mod certipulse {
    use super::*;

    pub fn issue_certificate(
        ctx: Context<IssueCertificate>,
        cert_id: String,
        student_name: String,
        course_name: String,
        cert_hash: [u8; 32],
    ) -> Result<()> {
        let cert = &mut ctx.accounts.certificate;
        cert.issuer = ctx.accounts.authority.key();
        cert.cert_id = cert_id;
        cert.student_name = student_name;
        cert.course_name = course_name;
        cert.cert_hash = cert_hash;
        cert.issued_at = Clock::get()?.unix_timestamp;

        msg!("CertiPulse: Certificate {} issued successfully.", cert.cert_id);
        Ok(())
    }

    pub fn verify_certificate(_ctx: Context<VerifyCertificate>, cert_id: String) -> Result<()> {
        msg!("CertiPulse: Certificate {} query executed.", cert_id);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(cert_id: String)]
pub struct IssueCertificate<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 64 + 64 + 64 + 32 + 8,
        seeds = [b"certificate", cert_id.as_bytes()],
        bump
    )]
    pub certificate: Account<'info, CertificateAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(cert_id: String)]
pub struct VerifyCertificate<'info> {
    #[account(
        seeds = [b"certificate", cert_id.as_bytes()],
        bump
    )]
    pub certificate: Account<'info, CertificateAccount>,
}

#[account]
pub struct CertificateAccount {
    pub issuer: Pubkey,
    pub cert_id: String,
    pub student_name: String,
    pub course_name: String,
    pub cert_hash: [u8; 32],
    pub issued_at: i64,
}
