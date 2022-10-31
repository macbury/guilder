import { createSlice, createEntityAdapter, createSelector } from '@reduxjs/toolkit';


type Errors = { [key:string]: string[] }

export type CategoryFormValue = {
  name: string
}

export interface ICategoriesState {
  loading: boolean,
  form: {
    visible: boolean,
    value: CategoryFormValue,
    categoryId: number,
    errors: Errors,
  }
}

const initialState = {
  loading: false,
  form: {
    visible: false,
    errors: {},
    categoryId: null,
    value: { name: '' }
  }
};

export const categoriesSlice = createSlice({
  name: 'categories',
  initialState,
  reducers: {
    setLoading: (state, { payload } : { payload: boolean }) => {
      state.loading = payload
    },

    setCategoryId: (state, { payload } : { payload: number }) => {
      state.form.categoryId = payload
    },

    setErrors: (state, { payload } : { payload: Errors }) => {
      state.form.errors = payload
    },

    resetForm: (state) => {
      state.form.errors = {};
      state.form.categoryId = null;
      state.form.value.name = '';
    },

    openForm: (state) => {
      state.form.visible = true;
    },

    updateForm: (state, { payload } : { payload: CategoryFormValue }) => {
      state.form.value = payload;
    },

    closeForm: (state) => {
      state.form.visible = false;
    }
  }
});

export const actions = categoriesSlice.actions;
export default categoriesSlice.reducer;
