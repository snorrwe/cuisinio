import { Component, OnInit } from '@angular/core';
import { ShortRecipe } from './model/recipe.model';

@Component({
    selector: 'cuisinio-recipe',
    templateUrl: './recipes.component.html',
    styleUrls: ['./recipes.component.css']
})
export class RecipesComponent implements OnInit {

    selectedId: string;

    constructor() { }

    ngOnInit() {
    }

    onSelect(recipe: ShortRecipe) {
        this.selectedId = recipe._id;
    }
}
