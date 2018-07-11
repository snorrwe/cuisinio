import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterModule, Routes } from '@angular/router';

import { RecipeService } from './services/recipe.service';
import { RecipeListComponent } from './recipe-list/recipe-list.component';
import { RecipeComponent } from './recipe.component';

const routes: Routes = [
    {
        path: ''
        , component: RecipeComponent
    }
];

@NgModule({
    imports: [
        CommonModule,
        RouterModule.forChild(routes)
    ],
    declarations: [RecipeListComponent, RecipeComponent],
    providers: [RecipeService]
})
export class RecipeModule { }
