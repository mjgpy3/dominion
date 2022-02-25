import ReactDOM from "react-dom";
import React from "react";
import init, * as Dominion from "./node_modules/dominion/dominion.js";
import SuperTreeview from "react-super-treeview";

function SetupGenerator({ unselectedExpansionCards, projectCounts, generate }) {
  const [includes, setIncludes] = React.useState(unselectedExpansionCards.slice());
  const [bans, setBans] = React.useState(unselectedExpansionCards.slice());
  const [expansions, setExpansions] = React.useState(unselectedExpansionCards.slice());
  const [projectCount, setProjectCount] = React.useState(null);
  const [generated, setGenerated] = React.useState(null);

  const nullIfEmpty = (vs) => (vs.length ? vs : null);

  const checkedChildren = (tree) =>
    nullIfEmpty(
      tree.flatMap((exp) =>
        exp.children.filter((v) => v.isChecked).map((v) => v.id)
      )
    );

  return (
    <div>
      <h1>Include Cards</h1>
      <SuperTreeview
        isDeletable={() => false}
        isCheckable={(_, depth) => depth > 0}
        isExpandable={(_, depth) => depth < 1}
        data={includes}
        onUpdateCb={setIncludes}
      />
      <h1>Ban Cards</h1>
      <SuperTreeview
        isDeletable={() => false}
        isCheckable={(_, depth) => depth > 0}
        isExpandable={(_, depth) => depth < 1}
        data={bans}
        onUpdateCb={setBans}
      />
      <h1>Expansion Pool</h1>
      <SuperTreeview
        isDeletable={() => false}
        isExpandable={() => false}
        data={expansions}
        onUpdateCb={setExpansions}
      />
      <h1>Project Count</h1>
      <input
        type="radio"
        value="random"
        id="random"
        onChange={() => setProjectCount(null)}
        name="project-count"
        checked={projectCount === null}
      />
      <label for="random">Random (from expansions)</label>

      {projectCounts.map((count) => (
        <>
          <input
            type="radio"
            value={count}
            id={`count-${count}`}
            onChange={() => setProjectCount(count)}
            name="project-count"
            checked={projectCount === count}
          />
          <label for={`count-${count}`}>{count}</label>
        </>
      ))}

      <br />
      <button
        onClick={() =>
          setGenerated(
            generate({
              project_count: projectCount,
              include_expansions: nullIfEmpty(
                expansions.filter((v) => v.isChecked).map((e) => e.id)
              ),
              include_cards: checkedChildren(includes),
              ban_cards: checkedChildren(bans),
            })
          )
        }
      >
        Generate!
      </button>
      <br />
      <pre>{generated && JSON.stringify(generated, null, 2)}</pre>
    </div>
  );
}

init().then(() => {
  console.log(
    Dominion.gen_setup_js({
      ban_cards: [],
      include_cards: ["YoungWitch"],
    }),
    Dominion.kingdom_cards_js(),
    Dominion.expansions_js(),
    Dominion.project_counts_js()
  );

  console.log(Dominion.expansion_cards_js());

  const unselectedExpansionCards = Object.entries(
    Dominion.expansion_cards_js()
  ).map(([exp, cards]) => ({
    name: exp,
    id: exp,
    children: cards.map((card) => ({
      name: card,
      id: card,
    })),
  }));

  console.log(unselectedExpansionCards);

  const app = document.getElementById("setup-generator");
  ReactDOM.render(
    <SetupGenerator
      unselectedExpansionCards={unselectedExpansionCards}
      projectCounts={Dominion.project_counts_js()}
      generate={Dominion.gen_setup_js}
    />,
    app
  );
});
