import _uniqueId from 'lodash/uniqueId'
import _differenceBy from 'lodash/differenceBy'

const DEFAULT_INFO_TIMEOUT = 5000
const DEFAULT_ERROR_TIMEOUT = 0 // persistent

const initialState = {
  errors: []
  , infos: []
}

export const alerts = {
  namespaced: false
  , state: initialState
  , getters: {
    hasError: state => state.errors.length > 0
    , hasInfo: state => state.infos.length > 0
    , errors: state => state.errors
    , infos: state => state.infos
  }
  , actions: {
    error({ commit }, { error, context, timeout }) {
      commit('addError', { error, context, timeout })
      console.error(error)
      return error
    }
    , clearError({ commit }, { id }){
      commit('removeError', id)
    }
    , clearErrors({ commit }) {
      commit('clearErrors')
    }
    , info({ commit }, { message, timeout }) {
      commit('addInfo', { message, timeout })
    }
    , clearInfo({ commit }, { id }){
      commit('removeInfo', id)
    }
    , clearInfos({ commit }) {
      commit('clearInfos')
    }
  }
  , mutations: {
    clearAll(state) {
      state.errors = []
      state.infos = []
    }
    , addError(state, { error, context, timeout }){
      state.errors.push({
        id: _uniqueId('error-')
        , error
        , context
        , timeout: timeout || DEFAULT_ERROR_TIMEOUT
      })
    }
    , removeError(state, id){
      state.errors = _differenceBy(state.errors, [{ id }], 'id')
    }
    , clearErrors(state){
      state.errors = []
    }
    , addInfo(state, { message, timeout }){
      state.infos.push({
        id: _uniqueId('info-')
        , message
        , timeout: timeout || DEFAULT_INFO_TIMEOUT
      })
    }
    , removeInfo(state, id){
      state.infos = _differenceBy(state.infos, [{ id }], 'id')
    }
    , clearInfos(state){
      state.infos = []
    }
  }
}
