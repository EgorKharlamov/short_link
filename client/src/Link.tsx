import type { Component } from 'solid-js';
import {useParams} from "@solidjs/router";
import {createEffect} from "solid-js";
import {Api} from "./api";

type Params = {
  id: string;
}

const Link: Component = () => {
  const {id} = useParams<Params>();
  createEffect(async () => {
    const res = await Api.getRedirectUrl(id)
    window.location.replace(res.data);
  })
  return <></>
};

export default Link;
