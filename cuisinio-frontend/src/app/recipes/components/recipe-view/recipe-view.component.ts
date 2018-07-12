import { Component, OnInit, Input, OnChanges } from '@angular/core';
import { RecipeService } from '../../services/recipe.service';
import { RecipeDTO } from '../../model/recipe.model';

@Component({
    selector: 'cuisinio-recipe-view',
    templateUrl: './recipe-view.component.html',
    styleUrls: ['./recipe-view.component.css']
})
export class RecipeViewComponent implements OnInit, OnChanges {
    @Input() recipeId: string;

    private _recipe: RecipeDTO;
    public get recipe() {
        return this._recipe && this._recipe.recipe;
    }

    constructor(private recipeService: RecipeService) { }

    ngOnInit() {
        this.refresh();
    }

    ngOnChanges() {
        this.refresh();
    }

    private refresh() {
        if (this.recipeId) {
            this.recipeService.getRecipeById(this.recipeId)
                .then(result => this._recipe = result);
        }
    }
}
