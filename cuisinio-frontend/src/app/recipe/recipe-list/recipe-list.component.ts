import { Component, OnInit } from '@angular/core';
import { RecipeService } from '../services/recipe.service';

@Component({
    selector: 'cuisinio-recipe-list',
    templateUrl: './recipe-list.component.html',
    styleUrls: ['./recipe-list.component.css']
})
export class RecipeListComponent implements OnInit {

    recipes: any[];

    constructor(private recipeService: RecipeService) {
        this.recipeService.getRecipes()
            .then(result => this.recipes = result);
    }

    ngOnInit() {
    }

}
