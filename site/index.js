import ReactDOM from "react-dom";
import React from "react";
import init, * as Dominion from "./node_modules/dominion/dominion.js";
import SuperTreeview from "react-super-treeview";

function SetupGenerator({
  makeUnselectedExpansionCards,
  projectCounts,
  baneCounts,
  generate,
  cardExpansions,
}) {
  const sortByName = (vs) => vs.sort((a, b) => a.name.localeCompare(b.name));

  const [includes, setIncludes] = React.useState(
    sortByName(makeUnselectedExpansionCards("includes"))
  );
  const [bans, setBans] = React.useState(
    sortByName(makeUnselectedExpansionCards("bans"))
  );
  const [expansions, setExpansions] = React.useState(
    sortByName(makeUnselectedExpansionCards("expansions"))
  );
  const [projectCount, setProjectCount] = React.useState(null);
  const [baneCount, setBaneCount] = React.useState(null);
  const [setup, setSetup] = React.useState(null);
  const [error, setError] = React.useState(null);

  const nullIfEmpty = (vs) => (vs.length ? vs : null);

  const checkedChildren = (tree) =>
    nullIfEmpty(
      tree.flatMap((exp) =>
        exp.children.filter((v) => v.isChecked).map((v) => v.name)
      )
    );

  const includedCards = checkedChildren(includes);
  const bannedCards = checkedChildren(bans);

  return (
    <div>
      <h1>
        Include Cards {includedCards && "(" + includedCards.join(", ") + ")"}
      </h1>
      <SuperTreeview
        isDeletable={() => false}
        isCheckable={(_, depth) => depth > 0}
        isExpandable={(_, depth) => depth < 1}
        data={includes}
        onUpdateCb={setIncludes}
      />
      <h1>Ban Cards {bannedCards && "(" + bannedCards.join(", ") + ")"}</h1>
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
      <label htmlFor="random">Random (from expansions)</label>

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
          <label htmlFor={`count-${count}`}>{count}</label>
        </>
      ))}

      <h1>Bane Count (Custom/Experimental Expansion)</h1>
      {baneCounts.map((count) => (
        <>
          <input
            type="radio"
            value={count}
            id={`bane-count-${count}`}
            onChange={() => setBaneCount(count)}
            name="bane-count"
            checked={baneCount === count}
          />
          <label htmlFor={`bane-count-${count}`}>{count}</label>
        </>
      ))}

      <br />
      <button
        onClick={() => {
          try {
            setSetup(
              generate({
                project_count: projectCount,
                bane_count: baneCount,
                include_expansions: nullIfEmpty(
                  expansions.filter((v) => v.isChecked).map((e) => e.name)
                ),
                include_cards: includedCards,
                ban_cards: bannedCards,
              })
            );
          } catch (e) {
            setError(e);
          }
        }}
      >
        Generate!
      </button>
      <br />
      {error && Dominion.gen_error_js(error)}
      {setup && <Setup setup={setup} cardExpansions={cardExpansions} />}
    </div>
  );
}

function Setup({ setup, cardExpansions }) {
  const cardsByExpansion = {};

  const usedExpansions = new Set();

  const spaces = (card) => card.replaceAll(/([A-Z])/g, " $1").trim();

  const formatCard = (card) => {
    if (card === setup.bane_card) {
      return `${spaces(card)} (Bane)`;
    }
    if (card in setup.bane_cards && setup.bane_cards[card] === "Zebra") {
      return `${spaces(card)} (${spaces(setup.bane_cards[card])} with ${
        setup.second_zebra
      })`;
    }
    if (card in setup.bane_cards) {
      return `${spaces(card)} (${spaces(setup.bane_cards[card])})`;
    }

    return spaces(card);
  };

  Dominion.setup_kingdom_cards_js(setup).forEach((kc) => {
    const expansions = cardExpansions[kc].sort().join("/");
    usedExpansions.add(expansions);
    cardsByExpansion[expansions] = (cardsByExpansion[expansions] || [])
      .concat([kc])
      .sort();
  });

  const usedExpansionsSorted = Array.from(usedExpansions).sort();

  return (
    <>
      <h1>Kingdom</h1>
      <div
        id="kingdom-grid"
        style={{ display: "grid", "grid-template-columns": "auto auto" }}
      >
        {usedExpansionsSorted.map((expansion) => (
          <>
            <div className="expansion">{expansion}</div>
            <div className="expansion-cards">
              {cardsByExpansion[expansion].map(formatCard).join(", ")}
            </div>
          </>
        ))}
      </div>
      {setup.project_cards.length > 0 && (
        <>
          <h1>Projects</h1>
          <ul>
            {setup.project_cards.map((project) => (
              <li className="project-card">{project}</li>
            ))}
          </ul>
        </>
      )}
      <h2>Hists</h2>
      <pre>{Dominion.hists_js(setup)}</pre>
    </>
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
      baneCounts={Dominion.bane_counts_js()}
      generate={Dominion.gen_setup_js}
    />,
    app
  );
});
