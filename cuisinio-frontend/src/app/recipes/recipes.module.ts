import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterModule, Routes } from '@angular/router';
import { FormsModule, ReactiveFormsModule } from '@angular/forms';

import { RecipeService } from './services/recipe.service';
import { RecipeListComponent } from './components/recipe-list/recipe-list.component';
import { RecipesComponent } from './recipes.component';
import { RecipeViewComponent } from './components/recipe-view/recipe-view.component';
import { NewRecipeComponent } from './components/new-recipe/new-recipe.component';

const routes: Routes = [
  {
    path: ''
    , component: RecipesComponent
  }
  , {
    path: 'new'
    , component: NewRecipeComponent
  }
];

@NgModule({
  imports: [
    CommonModule,
    RouterModule.forChild(routes),
    FormsModule,
    ReactiveFormsModule
  ],
  declarations: [RecipeListComponent, RecipesComponent, RecipeViewComponent, NewRecipeComponent],
  providers: [RecipeService]
})
export class RecipeModule { }
