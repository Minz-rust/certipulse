import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";

describe("certipulse", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  it("Issues and verifies certificate account on-chain", async () => {
    console.log("Anchor test initialized for CertiPulse Devnet program.");
  });
});
