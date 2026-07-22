import en from './en';
import fr from './fr';

export type Lang = 'en' | 'fr';
export const dictionaries = { en, fr } as const;
export type Dict = typeof en;

class LangState {
  current = $state<Lang>('fr');
  toggle() {
    this.current = this.current === 'fr' ? 'en' : 'fr';
  }
  get t(): Dict {
    return dictionaries[this.current];
  }
}

export const lang = new LangState();
