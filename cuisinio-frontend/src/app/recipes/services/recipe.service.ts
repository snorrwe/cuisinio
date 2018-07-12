import { Injectable } from '@angular/core';
import { Http } from '@angular/http';

import { ShortRecipe, RecipeDTO, Recipe } from '../model/recipe.model';

const API_BASE_URL = 'http://localhost:8000'; // TODO: abstract

@Injectable()
export class RecipeService {

  constructor(private http: Http) { }

  getRecipes(): Promise<ShortRecipe[]> {
    return this.http.get(API_BASE_URL + '/recipes')
      .toPromise()
      .then(response => {
        const result = response.json();
        console.log(result);
        return result;
      });
  }

  getRecipeById(id: string): Promise<RecipeDTO> {
    return this.http.get(API_BASE_URL + '/recipe/' + id)
      .toPromise()
      .then(response => {
        const result = response.json();
        console.log(result);
        return result;
      });
  }

  postRecipe(recipe: Recipe): Promise<RecipeDTO> {
    return this.http.post(API_BASE_URL + '/recipe', recipe)
      .toPromise()
      .then(response => {
        const result = response.json();
        console.log(result);
        return result;
      });
  }
}
