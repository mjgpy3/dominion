import ReactDOM from "react-dom";
import React from "react";
import init, * as Dominion from "./node_modules/dominion/dominion.js";

function LikeButton() {
  const [liked, setLiked] = React.useState(false);

  if (liked) {
    return "You liked this.";
  }

  return <button onClick={() => setLiked(true)} />;
}

init().then(() => {
  console.log(
    Dominion.gen_setup_js({}),
    Dominion.kingdom_cards_js(),
    Dominion.expansions_js(),
    Dominion.expansion_cards_js(),
    Dominion.project_counts_js()
  );

  const app = document.getElementById("setup-generator");
  ReactDOM.render(<LikeButton />, app);
});
