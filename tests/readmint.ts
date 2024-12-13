import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Readmint } from "../target/types/readmint";
import { assert, expect } from "chai";

describe("readmint", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Readmint as Program<Readmint>;

  let provider = anchor.getProvider() as anchor.AnchorProvider;

  let author = anchor.web3.Keypair.generate();
  let user = anchor.web3.Keypair.generate();

  before(async () => {
    const signature = await provider.connection.requestAirdrop(
      author.publicKey,
      anchor.web3.LAMPORTS_PER_SOL * 3
    );
    await provider.connection.confirmTransaction(signature);

    const signature1 = await provider.connection.requestAirdrop(
      user.publicKey,
      anchor.web3.LAMPORTS_PER_SOL * 3
    );
    await provider.connection.confirmTransaction(signature1);
  });

  it("Create a Book", async () => {
    await program.methods
      .createBook("test", "test", new anchor.BN(100))
      .accounts({
        author: author.publicKey,
      })
      .signers([author])
      .rpc()
      .catch((e) => {
        console.log(e);
      });

    const [bookPda, bump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("test"), Buffer.from("test")],
      program.programId
    );

    const account = await program.account.book.fetch(bookPda);
    expect(account.author.toString()).to.equal(author.publicKey.toString());
    expect(account.title).to.equal("test");
    expect(account.totalPages.toNumber()).to.equal(100);
  });

  it("Create a User", async () => {
    await program.methods
      .createUser()
      .accounts({
        owner: user.publicKey,
      })
      .signers([user])
      .rpc();

    const [userPda, userBump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("book"), user.publicKey.toBuffer()],
      program.programId
    );
    const account = await program.account.user.fetch(userPda);
    expect(account.tokenBalance.toNumber()).to.equal(0);
  });

  it("Create a UserBook and add it to the User", async () => {
    await program.methods
      .addBookToUser("test", "test")
      .accounts({
        owner: user.publicKey,
      })
      .signers([user])
      .rpc();

    const [userPda, userBump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("book"), user.publicKey.toBuffer()],
      program.programId
    );
    const userAccount = await program.account.user.fetch(userPda);

    const [bookPda, bump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("test"), Buffer.from("test")],
      program.programId
    );

    const [userBookPda, userBookBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [userPda.toBuffer(), bookPda.toBuffer()],
        program.programId
      );
    const userBookAccount = await program.account.userBook.fetch(userBookPda);

    expect(userBookAccount.currentPage.toNumber()).to.equal(0);
  });

  it("Update UserBook", async () => {
    await program.methods
      .updateUserBook(new anchor.BN(10), "test", "test")
      .accounts({
        owner: user.publicKey,
      })
      .signers([user])
      .rpc()
      .catch((e) => console.log(e));

    const [userPda, userBump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("book"), user.publicKey.toBuffer()],
      program.programId
    );
    const userAccount = await program.account.user.fetch(userPda);

    const [bookPda, bump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("test"), Buffer.from("test")],
      program.programId
    );

    const [userBookPda, userBookBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [userPda.toBuffer(), bookPda.toBuffer()],
        program.programId
      );
    const userBookAccount = await program.account.userBook.fetch(userBookPda);

    expect(userBookAccount.currentPage.toNumber()).to.equal(10);
  });
});
