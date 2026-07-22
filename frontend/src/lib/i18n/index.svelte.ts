import en from './en';
import fr from './fr';

export type Lang = 'en' | 'fr';
export type Dict = typeof en;
export const dictionaries: Record<Lang, Dict> = { en, fr };

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
