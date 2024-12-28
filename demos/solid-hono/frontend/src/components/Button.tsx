import type { Component, JSX } from "solid-js";

interface Props {
  onClick: () => void;
  children: JSX.Element;
}

const Button: Component<Props> = (props) => (
  <button
    class="text-white font-semibold bg-blue-500 px-4 py-2 rounded-md mt-4 hover:bg-blue-600"
    onClick={props.onClick}
  >
    {props.children}
  </button>
);

export default Button;
