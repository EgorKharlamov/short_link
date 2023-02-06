import type {Component} from 'solid-js';
import {Routes} from "./routes";
import {Toaster} from "solid-toast";

const App: Component = () => {
  return <>
      <Toaster/>
      <Routes/>
    </>

};

export default App;
