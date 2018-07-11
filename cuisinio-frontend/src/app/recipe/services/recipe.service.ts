import { Injectable } from '@angular/core';
import { Http } from '@angular/http';

import { Recipe } from '../model/recipe';

const API_BASE_URL = 'http://localhost:8000'; // TODO: abstract

@Injectable()
export class RecipeService {

    constructor(private http: Http) { }

    getRecipes(): Promise<Recipe[]> {
        return this.http.get(API_BASE_URL + '/recipes')
            .toPromise()
            .then(response => {
                const result = response.json();
                console.log(result);
                return result;
            });
    }

}
