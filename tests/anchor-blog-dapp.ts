import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { AnchorBlogDapp } from "../target/types/anchor_blog_dapp";
import { PublicKey, SystemProgram } from '@solana/web3.js';
import { LAMPORTS_PER_SOL } from '@solana/web3.js';
type Blog = {
  authority: PublicKey;
  bump: number;
  postCount: number;
}

type Post = {
  author: PublicKey,
  slug: String,
  title: String,
  content: String,
}

describe("anchor-blog-dapp", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.AnchorBlogDapp as Program<AnchorBlogDapp>;

  it("Is initialized!", async () => {
    // Add your test here.
    const keypair = anchor.web3.Keypair.generate();
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(keypair.publicKey, LAMPORTS_PER_SOL * 10), "confirmed"
    );
    const [blogAccount, blogBump] = await PublicKey.findProgramAddress(
      [
        Buffer.from("blog"), 
        keypair.publicKey.toBuffer()
      ],
      program.programId
    );
    const blog: Blog = {
      authority: keypair.publicKey,
      bump: blogBump,
      postCount: 0,
    };
    await program.rpc.initializeBlog(blog, {
      accounts: {
        blogAccount: blogAccount,
        authority: keypair.publicKey,
        systemProgram: SystemProgram.programId
      },
      signers: [keypair]
    });

    const blogAccountData: Blog = await program.account.blog.fetchNullable(blogAccount);
    console.log(blogAccountData);

    const [postAccount, postBump] = await PublicKey.findProgramAddress(
      [
        Buffer.from("post"),
        blogAccount.toBuffer(),
        Buffer.from("slug-1"),
      ],
      program.programId
    );

    const keypair1 = anchor.web3.Keypair.generate();
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(keypair1.publicKey, LAMPORTS_PER_SOL * 10), "confirmed"
    );
    const post: Post = {
      author: keypair1.publicKey,
      slug: "slug-1",
      title: "first post",
      content: "hello world!"
    };
    await program.rpc.createPost(post, {
      accounts: {
        postAccount,
        authority: keypair1.publicKey,
        blogAccount,
        systemProgram: SystemProgram.programId
      },
      signers: [keypair1]
    });

    const postData = await program.account.post.fetchNullable(postAccount);
    console.log(postData);
  });
});
