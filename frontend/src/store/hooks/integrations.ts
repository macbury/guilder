import React, { useEffect } from 'react';
import axios from 'axios';
import { useAppDispatch, useAppSelector, RootState } from '..';
import useAbortCallback from '../../hooks/useAbortCallback';
import { showError, showSuccess } from '../../toasts';
import {
  integrationsAdapter,
  actions,
  Integration,
  IntegrationFormValue,
} from '../slices/integrations';
import { api } from '../api';

export const {
  selectAll,
  selectById
} = integrationsAdapter.getSelectors<RootState>((state) => state.integrations);

export const fetchAll = (signal : AbortSignal) => async (dispatch, getState) => {
  dispatch(actions.setLoading(true))

  try {
    const { data: { integrations } } = await axios.get("/api/integrations", { signal });
    dispatch(actions.setIntegrations(integrations))
  } catch (e) {
    if (e.name !== "AbortError") {
      console.error("Aborted fetching", e);
    } else {
      showError(e);
    }
  } finally {
    dispatch(actions.setLoading(false))
  }
}

export const sync = (integrationId : number) => async (dispatch, getState) => {
  dispatch(actions.setLoading(true))

  try {
    const { data: { success } } = await axios.post(`/api/integrations/${integrationId}/sync`);
    if (success) {
      dispatch(api.util.invalidateTags(['Bond']));
      showSuccess("Integration synchronization finished!")
    } else {
      showError("Integration synchronization failed")
    }
  } catch (e) {
    showError(e);
  } finally {
    dispatch(actions.setLoading(false))
  }
}

export const save = () => async (dispatch, getState) => {
  dispatch(actions.setLoading(true))

  try {
    const { integrations: { form: { value } } } : RootState = getState();

    const { data: { success, resource, errors } } = await axios.post(`/api/integrations`, value);
    if (success) {
      dispatch(actions.setIntegration(resource));
      dispatch(actions.closeForm());
      showSuccess("Saved Integration!");
      dispatch(sync(resource.id))
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
    const { integrations: { form: { value, integrationId } } } : RootState = getState();

    const { data: { success, resource, errors } } = await axios.put(`/api/integrations/${integrationId}`, value);
    if (success) {
      dispatch(actions.setIntegration(resource));
      dispatch(actions.closeForm());
      showSuccess("Updated account!");
      dispatch(sync(resource.id))
    } else {
      dispatch(actions.setErrors(errors))
    }
  } catch (e) {
    showError(e);
  } finally {
    dispatch(actions.setLoading(false))
  }
}

export const edit = (integrationId : number) => async (dispatch, getState) => {
  dispatch(actions.setLoading(true))

  try {
    const { data: { integration } } = await axios.get(`/api/integrations/${integrationId}`);

    if (integration) {
      dispatch(actions.resetForm())
      dispatch(actions.setIntegrationId(integrationId))
      dispatch(actions.updateForm(integration));
      dispatch(actions.openForm())
    } else {
      console.error('Could not find integration....')
    }
  } catch (error) {
    showError(error.toString())
  } finally {
    dispatch(actions.setLoading(false))
  }
}

export const destroy = (integrationID : number) => async (dispatch, getState) => {
  dispatch(actions.setLoading(true))

  try {
    await axios.delete(`/api/integrations/${integrationID}`);
    dispatch(actions.removeIntegration(integrationID))
    showSuccess("Removed integration!")
  } catch (e) {
    showError(e);
  } finally {
    dispatch(actions.setLoading(false))
  }
}

export const newForm = () => (dispatch) => {
  dispatch(actions.resetForm())
  dispatch(actions.openForm())
}

export function useLoadIntegrations() {
  const dispatch = useAppDispatch();
  const request = useAbortCallback((signal) => {
    dispatch(fetchAll(signal));
  }, [dispatch])

  useEffect(() => {
    return request()
  }, [])
}

export function useIntegration(integrationId : number) : Integration {
  return useAppSelector(state => selectById(state, integrationId));
}

export function useIntegrationsState() {
  const {
    integrations,
    form,
    loading
  } = useAppSelector((state) => ({
    loading: state.integrations.loading,
    integrations: selectAll(state),
    form: state.integrations.form
  }));

  const dispatch = useAppDispatch();

  return {
    state: {
      integrations, loading, form
    },
    actions: {
      setIntegration: (integration : Integration) => dispatch(actions.setIntegration(integration)),
      updateForm: (value : IntegrationFormValue) => dispatch(actions.updateForm(value)),
      closeForm: () => dispatch(actions.closeForm()),
      newIntegration: () => dispatch(newForm()),
      sync: (id) => dispatch(sync(id)),
      editIntegration: (id) => dispatch(edit(id)),
      destroy: (id) => dispatch(destroy(id)),
      save: () => dispatch(save()),
      update: () => dispatch(update())
    }
  }
}
