/* @refresh reload */
import { render } from 'solid-js/web';
import { Router } from "@solidjs/router";
import './index.css';
import App from './App';
import AuthProvider from "./shared/context/auth.context";
import UiProvider from "./shared/context/ui.context";
import bg from "./assets/bg.jpg";

render(() =>
  <Router>
    <UiProvider>
      <AuthProvider>
        <img class="bg-image" src={bg} alt=""/>
        <App/>
      </AuthProvider>
    </UiProvider>
  </Router>,
  document.getElementById('root') as HTMLElement
);
