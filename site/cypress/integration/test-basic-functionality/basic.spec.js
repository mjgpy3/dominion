describe("dominion", () => {
  beforeEach(() => {
    cy.visit("https://mjgpy3.github.io/dominion/");
  });

  it("allows users to force inclusion of a specific cards", () => {
    cy.get(".super-treeview-triangle-btn-right").click({ multiple: true });

    cy.get('label[for="includes-Sentry"]').click();
    cy.get('label[for="includes-Jester"]').click();

    cy.contains("Generate!").click();

    cy.get(".expansion-cards").contains("Sentry").should("exist");
    cy.get(".expansion-cards").contains("Jester").should("exist");
    cy.get(".expansion").contains("Base2").should("exist");
  });

  it("young witch implies a bane card", () => {
    cy.get(".super-treeview-triangle-btn-right").click({ multiple: true });

    cy.get('label[for="includes-YoungWitch"]').click();

    cy.contains("Generate!").click();

    cy.get(".expansion-cards").contains("(Bane)").should("exist");
  });

  it("allows users to block the inclusion of specific cards (via bans)", () => {
    cy.get(".super-treeview-triangle-btn-right").click({ multiple: true });

    cy.get('label[for="bans-Sentry"]').click();
    cy.get('label[for="bans-Poacher"]').click();
    cy.get('label[for="expansions-Base2"]').click();

    cy.contains("Generate!").click();

    cy.get(".expansion-cards").contains("Sentry").should("not.exist");
    cy.get(".expansion-cards").contains("Poacher").should("not.exist");
  });

  it("allows users to choose the pool of expansions", () => {
    cy.get('label[for="expansions-Guilds"]').click();
    cy.get('label[for="expansions-Seaside"]').click();

    cy.contains("Generate!").click();

    cy.get(".expansion").contains("Base").should("not.exist");
    cy.get(".expansion").contains("Cornucopia").should("not.exist");
    cy.get(".expansion").contains("Renaissance").should("not.exist");
    cy.get(".expansion").contains("Intrigue").should("not.exist");
  });

  it("does not allow a specific card to be banned and included", () => {
    cy.get(".super-treeview-triangle-btn-right").click({ multiple: true });

    cy.get('label[for="includes-Sentry"]').click();
    cy.get('label[for="bans-Sentry"]').click();
    cy.get('label[for="expansions-Base2"]').click();

    cy.contains("Generate!").click();

    cy.get(".expansion").should("not.exist");
  });

  [0, 1, 2].forEach((count) => {
    it(`allows users to force a specific project count of ${count}`, () => {
      cy.get(`#count-${count}`).click();

      cy.contains("Generate!").click();

      cy.get(".project-card").should("have.length", count);
    });
  });
});
