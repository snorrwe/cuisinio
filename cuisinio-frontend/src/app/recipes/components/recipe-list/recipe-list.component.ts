import { Component, OnInit, Output, EventEmitter, OnDestroy } from '@angular/core';
import { FormBuilder } from '@angular/forms';
import { ShortRecipe } from '../../model/recipe.model';
import { RecipeService } from '../../services/recipe.service';

@Component({
  selector: 'cuisinio-recipe-list',
  templateUrl: './recipe-list.component.html',
  styleUrls: ['./recipe-list.component.css']
})
export class RecipeListComponent implements OnInit, OnDestroy {

  recipes: ShortRecipe[];
  @Output('select') onSelectEventEmitter = new EventEmitter<ShortRecipe>();

  private refreshTicks: any = null;

  constructor(private recipeService: RecipeService) {
    this.refresh();
  }

  ngOnInit() {
    this.refreshTicks = setInterval(() => this.refresh(), 60000);
  }

  ngOnDestroy() {
    if (this.refreshTicks) {
      clearInterval(this.refreshTicks);
    }
  }

  refresh() {
    this.recipeService.getRecipes()
      .then(result => this.recipes = result);
  }

  select(recipe: ShortRecipe) {
    this.onSelectEventEmitter.emit(recipe);
  }
}
