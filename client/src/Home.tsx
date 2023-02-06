import type {Component} from 'solid-js';
import {createSignal} from "solid-js";
import {Api} from "./api";
import {toast} from "solid-toast";
import {appUrl} from "./config";

const App: Component = () => {
  const [longLink, setLongLink] = createSignal<string>('');
  const [shortLink, setShortLink] = createSignal<string | null>(null);
  const clickHandler = async () => {
    try {
      const link = new URL(longLink());
      if (appUrl.includes(link.host)) {
        setLongLink("");
        throw new Error("You can't use the same host!")
      }
      const res = await Api.sendLongLink(link.href);
      setShortLink(`${appUrl}/${res.data}`)
      toast.success('Yay! Short link is ready!');
    } catch (e: any) { // :'(
      toast.error(e.message)
    }
  }

  const onCopy = async () => {
    let link = shortLink()
    if (link){
      await navigator.clipboard.writeText(link);
      toast.success('Link copied!');
    }
  }
  return (
    <div class="flex w-full h-[100vh] justify-center flex-col">
      <div class="mx-auto relative">
        <div>
          <input type="text" placeholder="Your url" class="border rounded-lg mr-3 text-2xl"
                 onInput={(e) => setLongLink(e.currentTarget.value.trim())} value={longLink()}/>
          <button type="button"
                  class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded-lg text-2xl cursor-pointer"
                  onClick={clickHandler}
          >
            Short!
          </button>
        </div>
        {shortLink() &&
          <div class="mt-5">
            <span
              class="font-bold text-3xl bottom-[-50px] left-[115px] text-blue-700 mr-3"
            >
              {shortLink()}
            </span>
            <button type="button"
                    class="border-2 text-black font-bold py-2 px-4 rounded text-2xl cursor-pointer"
                    onClick={onCopy}
            >
              Copy
            </button>
          </div>
        }
      </div>
    </div>
  );
};

export default App;
