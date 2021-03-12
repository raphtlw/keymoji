import Head from "next/head";
import "emoji-mart/css/emoji-mart.css";
import { Picker } from "emoji-mart";
import { external } from "../utils/utils";

export default function IndexPage() {
  function onSelect(emoji: any) {
    console.log("Emoji selected");
    external.invoke(`COPY_EMOJI ${emoji.native}`);
  }

  return (
    <main>
      <Head>
        <title>Emoji Picker</title>
      </Head>
      <Picker set='apple' onSelect={onSelect} native={true} />
    </main>
  );
}
