import type { Component } from 'solid-js';
import { toast } from 'solid-toast';
import { createSignal, onMount, Show } from 'solid-js';
import clsx from 'clsx';
import { nanoid } from 'nanoid';
import { CustomInputEvent, KeyCode, TryCatchError } from '@/types';
import { appUrl } from '@/config';
import { Api } from '@/api';

const App: Component = () => {
  const [longLink, setLongLink] = createSignal<string>('');
  const [shortLink, setShortLink] = createSignal<string | null>(null);
  let input: HTMLInputElement;
  const uuidInput = nanoid();

  const clickShortHandler = async () => {
    try {
      if (!longLink().slice(0, 5).includes('http'))
        throw new Error('You must specify the protocol (http:// or https://)');

      const link = new URL(longLink());
      if (appUrl.includes(link.host)) {
        setLongLink('');
        throw new Error("You can't use the same host!");
      }
      const res = await Api.sendLongLink(link.href);
      setShortLink(`${appUrl}/${res.data}`);
      toast.success('Yay! Short link is ready!');
      await onCopy();
    } catch (e: TryCatchError) {
      toast.error(e.message);
    }
  };
  const inputHandler = (e: CustomInputEvent) => {
    setLongLink(e.currentTarget.value.trim());
  };
  const keyDownInputHandler = async (e: KeyboardEvent) => {
    if (e.code === KeyCode.Enter) await clickShortHandler();
  };

  const onCopy = async () => {
    const link = shortLink();
    if (link) {
      await navigator.clipboard.writeText(link);
      toast.success('Link copied!');
    }
  };

  onMount(() => input.focus());

  const buttonMainClasses = clsx(
    'w-full cursor-pointer rounded-lg bg-blue-500',
    'py-2 px-4 text-2xl font-bold text-white transition-all',
    'hover:bg-blue-700 md:w-[250px]'
  );
  const buttonSecondaryClasses = clsx(
    'cursor-pointer rounded border-2 py-2 px-4 text-2xl',
    'font-bold text-black transition-all hover:border-gray-300',
    'focus:border-gray-800 md:w-[250px]'
  );

  return (
    <div class='mx-auto flex h-[100vh] w-full  min-w-[300px] max-w-[1500px] flex-col justify-evenly'>
      <h3 class='block w-full px-3 text-center text-3xl md:px-9'>
        Link lifetime is <span class='font-bold'>24 hours</span>. Please
        remember this before using this service.
      </h3>
      <div class='relative mx-auto md:w-full'>
        <div class='flex w-full flex-col gap-3 px-3 md:flex-row md:px-9'>
          <input
            data-uid={uuidInput}
            type='text'
            placeholder='Your url'
            class='w-full rounded-lg border text-2xl'
            onInput={inputHandler}
            value={longLink()}
            onKeyDown={keyDownInputHandler}
            ref={input}
          />

          <button
            type='button'
            class={buttonMainClasses}
            onClick={clickShortHandler}
          >
            Short
          </button>
        </div>
        <Show
          keyed={true}
          when={shortLink()}
        >
          <div class='mt-5 flex w-full flex-col gap-3 px-3 md:flex-row md:px-9'>
            <span class='mr-3 w-full text-lg font-bold text-blue-700 md:text-3xl'>
              {shortLink()}
            </span>
            <button
              type='button'
              class={buttonSecondaryClasses}
              onClick={onCopy}
            >
              Copy
            </button>
          </div>
        </Show>
      </div>
    </div>
  );
};

export default App;
