import React, { useEffect, useMemo } from 'react';
import { bindActionCreators } from '@reduxjs/toolkit';
import axios from 'axios'
import { showError, showSuccess } from '../../toasts';
import {
  actions,
} from '../slices/categories';
import { useAppDispatch, useAppSelector, RootState } from '..';
import useAbortCallback from '../../hooks/useAbortCallback';
import { api } from '../api';

export const save = () => async (dispatch, getState) => {
  dispatch(actions.setLoading(true))

  try {
    const { categories: { form: { value } } } : RootState = getState();

    const { data: { success, resource, errors } } = await axios.post(`/api/categories`, value);
    if (success) {
      dispatch(api.util.invalidateTags(['Category']))
      dispatch(actions.closeForm());
      showSuccess("Saved category!");
    } else {
      dispatch(actions.setErrors(errors))
    }
  } catch (e) {
    showError(e);
  } finally {
    dispatch(actions.setLoading(false))
  }
}

export const update = () => async (dispatch, getState) => {
  dispatch(actions.setLoading(true))

  try {
    const { categories: { form: { value, categoryId } } } : RootState = getState();

    const { data: { success, resource, errors } } = await axios.put(`/api/categories/${categoryId}`, value);
    if (success) {
      dispatch(api.util.invalidateTags(['Category']))
      dispatch(actions.closeForm());
      showSuccess("Updated category!");
    } else {
      dispatch(actions.setErrors(errors))
    }
  } catch (e) {
    showError(e);
  } finally {
    dispatch(actions.setLoading(false))
  }
}

export const editCategory = (categoryId : number) => async (dispatch, getState) => {
  dispatch(actions.setLoading(true))

  try {
    const { data: { category } } = await axios.get(`/api/categories/${categoryId}`);

    if (category) {
      dispatch(actions.resetForm())
      dispatch(actions.setCategoryId(categoryId))
      dispatch(actions.updateForm(category));
      dispatch(actions.openForm())
    } else {
      console.error('Could not find category....')
    }
  } catch (error) {
    showError(error.toString())
  } finally {
    dispatch(actions.setLoading(false))
  }
}

export const destroy = (categoryId : number) => async (dispatch, getState) => {
  dispatch(actions.setLoading(true))

  try {
    await axios.delete(`/api/categories/${categoryId}`);
    dispatch(api.util.invalidateTags(['Category']))
    showSuccess("Removed category!")
  } catch (e) {
    showError(e);
  } finally {
    dispatch(actions.setLoading(false))
  }
}

export const newCategory = () => (dispatch) => {
  dispatch(actions.resetForm())
  dispatch(actions.openForm())
}


export function useCategoriesState() {
  const state = useAppSelector((state) => ({
    loading: state.categories.loading,
    form: state.categories.form
  }));

  const dispatch = useAppDispatch();
  const boundActions = useMemo(() => bindActionCreators({
    updateForm: actions.updateForm,
    closeForm: actions.closeForm,
    newCategory,
    editCategory,
    destroy,
    save,
    update,
  }, dispatch), [dispatch])

  return {
    state,
    actions: boundActions
  }
}
