import axios from 'axios'
import { actions } from '../slices/session';
import { showError, showSuccess } from '../../toasts';
import { useAppDispatch, useAppSelector } from '..';

export const refresh = () => async (dispatch, getState) => {
  dispatch(actions.setStatus('initializing'))

  try {
    const { data } = await axios.get("/api/me");
    dispatch(actions.setCurrentUser(data))
  } catch (e) {
    dispatch(actions.setCurrentUser(null))
  } finally {
    dispatch(actions.setStatus('normal'))
  }
}

export const logout = () => async (dispatch, getState) => {
  dispatch(actions.setStatus('initializing'))
  dispatch(actions.setCurrentUser(null))

  try {
    await axios.get("/api/logout");
  } finally {
    document.location.reload()
  }
}

export const signIn = (login : string, password: string) => async (dispatch, getState) => {
  dispatch(actions.setStatus('loading'))

  try {
    const { data: { success } } = await axios.post('/api/sign_in', { login, password })
    if (success) {
      dispatch(refresh())
      showSuccess("Signed in successfully");
    } else {
      console.log("Invalid login or password")
      showError("Invalid login or password");
    }
  } catch (e) {
    showError(e.toString());
  } finally {
    dispatch(actions.setStatus('normal'))
  }
}

export function useSignedIn() {
  const {
    user
  } = useAppSelector((state) => state.session);

  return !!user
}

export function useAuthenticationManager() {
  const state = useAppSelector((state) => state.session);
  const dispatch = useAppDispatch();

  return {
    ...state,
    signIn: (login, password) => dispatch(signIn(login, password)),
    logout: () => dispatch(logout()),
    refresh: () => dispatch(refresh())
  }
}
