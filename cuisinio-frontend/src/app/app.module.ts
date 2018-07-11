import { BrowserModule } from '@angular/platform-browser';
import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { HttpModule } from '@angular/http';

import { RecipeModule } from './recipe/recipe.module';
import { AppComponent } from './app.component';

const appRoutes: Routes = [
    {
        path: '',
        redirectTo: 'recipe',
        pathMatch: 'full'
    }
];

@NgModule({
    declarations: [
        AppComponent
    ],
    imports: [
        BrowserModule
        , RouterModule.forRoot(appRoutes)
        , RecipeModule
        , HttpModule
    ],
    providers: [],
    bootstrap: [AppComponent]
})
export class AppModule { }

