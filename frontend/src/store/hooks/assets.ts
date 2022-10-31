import React, { useMemo, useEffect } from 'react';
import axios from 'axios';
import { bindActionCreators, createSelector } from '@reduxjs/toolkit';
import { useAppDispatch, useAppSelector, RootState } from '..';
import useAbortCallback from '../../hooks/useAbortCallback';
import { showError, showSuccess } from '../../toasts';
import {
  assetsAdapter,
  actions,
  AssetFormValue,
  Asset,
} from '../slices/assets';
import { AssetsFilterOptions, fetchAll, destroy, fetch } from '../api/assets';

export const {
  selectAll,
  selectById
} = assetsAdapter.getSelectors<RootState>((state) => state.assets);

export const selectedEntitiesIds = createSelector((state : RootState) => state.assets.selectedIds, (selectedIds) => {
  return Object.keys(selectedIds).map((id) => {
    if (selectedIds[id]) {
      return id
    } else {
      return null
    }
  }).filter((id) => id != null)
})

export const selectFormValue = createSelector((state : RootState) => state.assets.form, (form) => form.value)

export const selectedCountSelector = createSelector(selectedEntitiesIds, (selectedIds) => {
  return selectedIds.length
})

export const fetchAsset = (ticker : string) => (dispatch) => {
  dispatch(actions.setCurrentAssetId(ticker))
  dispatch(fetch(ticker))
}

export const editAssets = () => (dispatch) => {
  dispatch(actions.resetForm())
  dispatch(actions.clearErrors())
  dispatch(actions.showForm())
}

export const massUpdate = () => async (dispatch, getState) => {
  dispatch(actions.setLoading(true));
  dispatch(actions.clearErrors())

  const state = getState();
  const selectedIds = selectedEntitiesIds(state);
  const form = selectFormValue(state);

  for (let index = 0; index < selectedIds.length; index++) {
    const ticker = selectedIds[index];

    try {
      const { data: { success, resource, errors } } = await axios.put(`/api/assets/${ticker}`, form);
      if (success) {
        dispatch(actions.deselect(ticker));
        dispatch(actions.updateAssets([resource]));
        showSuccess(`Updated asset: ${ticker}`);
      } else {
        dispatch(actions.setErrors(errors))
        return
      }
    } catch (error) {
      showError(error.toString())
      break;
    }
  }

  dispatch(actions.resetForm())
  dispatch(actions.hideForm())
  dispatch(actions.setLoading(false));
}

export const destroyAll = () => async (dispatch, getState) => {
  if (!confirm("Do you want to destroy selected assets?")) {
    return
  }

  dispatch(actions.setLoading(true));
  const selectedIds = selectedEntitiesIds(getState());

  for (let index = 0; index < selectedIds.length; index++) {
    const ticker = selectedIds[index];

    try {
      await axios.delete(`/api/assets/${ticker}`);

      dispatch(actions.removeAsset(ticker));
      showSuccess(`Removed asset: ${ticker}`);
    } catch (error) {
      showError(error.toString())
    }
  }

  dispatch(actions.clearSelection());
  dispatch(actions.setLoading(false));
}

export function useAssetsState() {
  const dispatch = useAppDispatch();
  const state = useAppSelector((state) => ({
    loading: state.assets.loading,
    assets: selectAll(state),
    asset: selectById(state, state.assets.currentAssetId || '-1'),
    selectedIds: state.assets.selectedIds,
    form: state.assets.form,
    selectedCount: selectedCountSelector(state)
  }));

  const boundActions = useMemo(() => bindActionCreators(({
    ...actions,
    editAssets,
    massUpdate,
    destroyAll,
    fetchAsset,
    destroy
  }), dispatch), [dispatch])

  return {
    state,
    actions: boundActions
  }
}

export function useSortableAssetsState(filter : AssetsFilterOptions) {
  const dispatch = useAppDispatch();
  const state = useAssetsState();

  const refresh = useAbortCallback((signal, params : AssetsFilterOptions) => {
    dispatch(fetchAll({ signal, params }))
  }, []);

  useEffect(() => {
    return refresh(filter);
  }, [filter])

  return state
}
