export interface Response {
  data: string;
  code: number;
}

export interface CustomHTMLInputElement {
  target: Element;
  currentTarget: HTMLInputElement;
}

export type CustomInputEvent = InputEvent & CustomHTMLInputElement;
// eslint-disable-next-line @typescript-eslint/no-explicit-any
export type TryCatchError = any;

export enum KeyCode {
  Enter = 'Enter',
}
