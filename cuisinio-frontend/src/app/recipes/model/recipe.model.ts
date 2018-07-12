export interface ShortRecipe {
  _id: string;
  name: string;
  description: string;
}

export interface RecipeDTO {
  _id: string;
  recipe: Recipe;
}

export interface Recipe {
    name: string;
    description: string;
    steps: Step[];
    graph: {
        edges: [2][]
    };
}

export interface Step {
  name: string;
  description: string;
  duration_ms: string;
}
