import ReactDOM from "react-dom";
import React from "react";
import init, * as Dominion from "./node_modules/dominion/dominion.js";
import SuperTreeview from "react-super-treeview";

function SetupGenerator({
  makeUnselectedExpansionCards,
  projectCounts,
  generate,
  cardExpansions,
}) {
  const [includes, setIncludes] = React.useState(
    makeUnselectedExpansionCards("includes")
  );
  const [bans, setBans] = React.useState(makeUnselectedExpansionCards("bans"));
  const [expansions, setExpansions] = React.useState(
    makeUnselectedExpansionCards("expansions")
  );
  const [projectCount, setProjectCount] = React.useState(null);
  const [setup, setSetup] = React.useState(null);

  const nullIfEmpty = (vs) => (vs.length ? vs : null);

  const checkedChildren = (tree) =>
    nullIfEmpty(
      tree.flatMap((exp) =>
        exp.children.filter((v) => v.isChecked).map((v) => v.name)
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
          setSetup(
            generate({
              project_count: projectCount,
              include_expansions: nullIfEmpty(
                expansions.filter((v) => v.isChecked).map((e) => e.name)
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
      <pre>{setup && JSON.stringify(setup, null, 2)}</pre>
    </div>
  );
}

init().then(() => {
  const expansionCards = Dominion.expansion_cards_js();
  const cardExpansions = Object.entries(expansionCards)
    .flatMap(([expansion, cards]) => cards.map((card) => [card, expansion]))
    .reduce(
      (res, [card, expansion]) => ({
        ...res,
        [card]: (res[card] || []).concat(expansion),
      }),
      {}
    );

    console.log(cardExpansions)

  const makeUnselectedExpansionCards = (idPrefix) =>
    Object.entries(expansionCards).map(([exp, cards]) => ({
      name: exp,
      id: `${idPrefix}-${exp}`,
      children: cards.map((card) => ({
        name: card,
        id: `${idPrefix}-${card}`,
      })),
    }));

  const app = document.getElementById("setup-generator");
  ReactDOM.render(
    <SetupGenerator
      makeUnselectedExpansionCards={makeUnselectedExpansionCards}
      cardExpansions={cardExpansions}
      projectCounts={Dominion.project_counts_js()}
      generate={Dominion.gen_setup_js}
    />,
    app
  );
});
