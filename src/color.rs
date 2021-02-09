#![allow(clippy::excessive_precision)]
use bevy::render::color::Color;

pub const DEC_0: f32 = 0.;
pub const HEX_00: f32 = DEC_0;
pub const DEC_11: f32 = 0.00334653561003506183624267578125;
pub const HEX_0B: f32 = DEC_11;
pub const DEC_19: f32 = 0.0065120910294353961944580078125;
pub const HEX_13: f32 = DEC_19;
pub const DEC_20: f32 = 0.0069954101927578449249267578125;
pub const HEX_14: f32 = DEC_20;
pub const DEC_21: f32 = 0.007499031722545623779296875;
pub const HEX_15: f32 = DEC_21;
pub const DEC_25: f32 = 0.00972121767699718475341796875;
pub const HEX_19: f32 = DEC_25;
pub const DEC_30: f32 = 0.012983030639588832855224609375;
pub const HEX_1E: f32 = DEC_30;
pub const DEC_32: f32 = 0.014443843625485897064208984375;
pub const HEX_20: f32 = DEC_32;
pub const DEC_34: f32 = 0.01599629223346710205078125;
pub const HEX_22: f32 = DEC_34;
pub const DEC_35: f32 = 0.01680737547576427459716796875;
pub const HEX_23: f32 = DEC_35;
pub const DEC_42: f32 = 0.023153364658355712890625;
pub const HEX_2A: f32 = DEC_42;
pub const DEC_43: f32 = 0.02415763027966022491455078125;
pub const HEX_2B: f32 = DEC_43;
pub const DEC_45: f32 = 0.02624122239649295806884765625;
pub const HEX_2D: f32 = DEC_45;
pub const DEC_46: f32 = 0.0273208916187286376953125;
pub const HEX_2E: f32 = DEC_46;
pub const DEC_47: f32 = 0.02842603810131549835205078125;
pub const HEX_2F: f32 = DEC_47;
pub const DEC_50: f32 = 0.0318960286676883697509765625;
pub const HEX_32: f32 = DEC_50;
pub const DEC_51: f32 = 0.03310476243495941162109375;
pub const HEX_33: f32 = DEC_51;
pub const DEC_60: f32 = 0.0451862029731273651123046875;
pub const HEX_3C: f32 = DEC_60;
pub const DEC_61: f32 = 0.0466650836169719696044921875;
pub const HEX_3D: f32 = DEC_61;
pub const DEC_63: f32 = 0.04970656335353851318359375;
pub const HEX_3F: f32 = DEC_63;
pub const DEC_64: f32 = 0.0512694679200649261474609375;
pub const HEX_40: f32 = DEC_64;
pub const DEC_65: f32 = 0.052860654890537261962890625;
pub const HEX_41: f32 = DEC_65;
pub const DEC_69: f32 = 0.0595112405717372894287109375;
pub const HEX_45: f32 = DEC_69;
pub const DEC_70: f32 = 0.0612460710108280181884765625;
pub const HEX_46: f32 = DEC_70;
pub const DEC_71: f32 = 0.063010029494762420654296875;
pub const HEX_47: f32 = DEC_71;
pub const DEC_72: f32 = 0.064803279936313629150390625;
pub const HEX_48: f32 = DEC_72;
pub const DEC_75: f32 = 0.07036010921001434326171875;
pub const HEX_4B: f32 = DEC_75;
pub const DEC_79: f32 = 0.078187428414821624755859375;
pub const HEX_4F: f32 = DEC_79;
pub const DEC_80: f32 = 0.080219827592372894287109375;
pub const HEX_50: f32 = DEC_80;
pub const DEC_82: f32 = 0.08437621593475341796875;
pub const HEX_52: f32 = DEC_82;
pub const DEC_85: f32 = 0.090841732919216156005859375;
pub const HEX_55: f32 = DEC_85;
pub const DEC_87: f32 = 0.09530748426914215087890625;
pub const HEX_57: f32 = DEC_87;
pub const DEC_90: f32 = 0.102241747081279754638671875;
pub const HEX_5A: f32 = DEC_90;
pub const DEC_92: f32 = 0.107023112475872039794921875;
pub const HEX_5C: f32 = DEC_92;
pub const DEC_95: f32 = 0.114435382187366485595703125;
pub const HEX_5F: f32 = DEC_95;
pub const DEC_96: f32 = 0.11697067320346832275390625;
pub const HEX_60: f32 = DEC_96;
pub const DEC_99: f32 = 0.124771840870380401611328125;
pub const HEX_63: f32 = DEC_99;
pub const DEC_100: f32 = 0.12743769586086273193359375;
pub const HEX_64: f32 = DEC_100;
pub const DEC_102: f32 = 0.13286833465099334716796875;
pub const HEX_66: f32 = DEC_102;
pub const DEC_104: f32 = 0.13843162357807159423828125;
pub const HEX_68: f32 = DEC_104;
pub const DEC_105: f32 = 0.141263306140899658203125;
pub const HEX_69: f32 = DEC_105;
pub const DEC_106: f32 = 0.14412848651409149169921875;
pub const HEX_6A: f32 = DEC_106;
pub const DEC_107: f32 = 0.1470272839069366455078125;
pub const HEX_6B: f32 = DEC_107;
pub const DEC_112: f32 = 0.16202940046787261962890625;
pub const HEX_70: f32 = DEC_112;
pub const DEC_113: f32 = 0.165132224559783935546875;
pub const HEX_71: f32 = DEC_113;
pub const DEC_114: f32 = 0.1682693958282470703125;
pub const HEX_72: f32 = DEC_114;
pub const DEC_119: f32 = 0.18447498977184295654296875;
pub const HEX_77: f32 = DEC_119;
pub const DEC_122: f32 = 0.194617807865142822265625;
pub const HEX_7A: f32 = DEC_122;
pub const DEC_123: f32 = 0.1980693042278289794921875;
pub const HEX_7B: f32 = DEC_123;
pub const DEC_124: f32 = 0.2015562355518341064453125;
pub const HEX_7C: f32 = DEC_124;
pub const DEC_127: f32 = 0.21223072707653045654296875;
pub const HEX_7F: f32 = DEC_127;
pub const DEC_128: f32 = 0.21586053073406219482421875;
pub const HEX_80: f32 = DEC_128;
pub const DEC_130: f32 = 0.223227977752685546875;
pub const HEX_82: f32 = DEC_130;
pub const DEC_133: f32 = 0.234550654888153076171875;
pub const HEX_85: f32 = DEC_133;
pub const DEC_134: f32 = 0.238397657871246337890625;
pub const HEX_86: f32 = DEC_134;
pub const DEC_135: f32 = 0.2422811985015869140625;
pub const HEX_87: f32 = DEC_135;
pub const DEC_136: f32 = 0.24620139598846435546875;
pub const HEX_88: f32 = DEC_136;
pub const DEC_138: f32 = 0.25415217876434326171875;
pub const HEX_8A: f32 = DEC_138;
pub const DEC_139: f32 = 0.2581829130649566650390625;
pub const HEX_8B: f32 = DEC_139;
pub const DEC_140: f32 = 0.262250721454620361328125;
pub const HEX_8C: f32 = DEC_140;
pub const DEC_142: f32 = 0.270497858524322509765625;
pub const HEX_8E: f32 = DEC_142;
pub const DEC_143: f32 = 0.2746773660182952880859375;
pub const HEX_8F: f32 = DEC_143;
pub const DEC_144: f32 = 0.2788943350315093994140625;
pub const HEX_90: f32 = DEC_144;
pub const DEC_147: f32 = 0.2917706966400146484375;
pub const HEX_93: f32 = DEC_147;
pub const DEC_148: f32 = 0.2961383163928985595703125;
pub const HEX_94: f32 = DEC_148;
pub const DEC_149: f32 = 0.300543844699859619140625;
pub const HEX_95: f32 = DEC_149;
pub const DEC_150: f32 = 0.304987370967864990234375;
pub const HEX_96: f32 = DEC_150;
pub const DEC_152: f32 = 0.313988745212554931640625;
pub const HEX_98: f32 = DEC_152;
pub const DEC_153: f32 = 0.318546831607818603515625;
pub const HEX_99: f32 = DEC_153;
pub const DEC_154: f32 = 0.3231432437896728515625;
pub const HEX_9A: f32 = DEC_154;
pub const DEC_158: f32 = 0.3419144451618194580078125;
pub const HEX_9E: f32 = DEC_158;
pub const DEC_160: f32 = 0.3515326976776123046875;
pub const HEX_A0: f32 = DEC_160;
pub const DEC_164: f32 = 0.3712377846240997314453125;
pub const HEX_A4: f32 = DEC_164;
pub const DEC_165: f32 = 0.3762622177600860595703125;
pub const HEX_A5: f32 = DEC_165;
pub const DEC_169: f32 = 0.3967553079128265380859375;
pub const HEX_A9: f32 = DEC_169;
pub const DEC_170: f32 = 0.4019778668880462646484375;
pub const HEX_AA: f32 = DEC_170;
pub const DEC_173: f32 = 0.4178851544857025146484375;
pub const HEX_AD: f32 = DEC_173;
pub const DEC_175: f32 = 0.42869055271148681640625;
pub const HEX_AF: f32 = DEC_175;
pub const DEC_176: f32 = 0.4341537058353424072265625;
pub const HEX_B0: f32 = DEC_176;
pub const DEC_178: f32 = 0.4452012479305267333984375;
pub const HEX_B2: f32 = DEC_178;
pub const DEC_179: f32 = 0.4507858455181121826171875;
pub const HEX_B3: f32 = DEC_179;
pub const DEC_180: f32 = 0.456411063671112060546875;
pub const HEX_B4: f32 = DEC_180;
pub const DEC_181: f32 = 0.4620770514011383056640625;
pub const HEX_B5: f32 = DEC_181;
pub const DEC_182: f32 = 0.4677838385105133056640625;
pub const HEX_B6: f32 = DEC_182;
pub const DEC_183: f32 = 0.473531544208526611328125;
pub const HEX_B7: f32 = DEC_183;
pub const DEC_184: f32 = 0.479320228099822998046875;
pub const HEX_B8: f32 = DEC_184;
pub const DEC_185: f32 = 0.48514997959136962890625;
pub const HEX_B9: f32 = DEC_185;
pub const DEC_186: f32 = 0.4910208880901336669921875;
pub const HEX_BA: f32 = DEC_186;
pub const DEC_188: f32 = 0.502886593341827392578125;
pub const HEX_BC: f32 = DEC_188;
pub const DEC_189: f32 = 0.50888144969940185546875;
pub const HEX_BD: f32 = DEC_189;
pub const DEC_191: f32 = 0.520995676517486572265625;
pub const HEX_BF: f32 = DEC_191;
pub const DEC_192: f32 = 0.52711522579193115234375;
pub const HEX_C0: f32 = DEC_192;
pub const DEC_193: f32 = 0.533276498317718505859375;
pub const HEX_C1: f32 = DEC_193;
pub const DEC_196: f32 = 0.5520114898681640625;
pub const HEX_C4: f32 = DEC_196;
pub const DEC_199: f32 = 0.57112491130828857421875;
pub const HEX_C7: f32 = DEC_199;
pub const DEC_203: f32 = 0.597201883792877197265625;
pub const HEX_CB: f32 = DEC_203;
pub const DEC_204: f32 = 0.603827416896820068359375;
pub const HEX_CC: f32 = DEC_204;
pub const DEC_205: f32 = 0.610495626926422119140625;
pub const HEX_CD: f32 = DEC_205;
pub const DEC_206: f32 = 0.617206633090972900390625;
pub const HEX_CE: f32 = DEC_206;
pub const DEC_208: f32 = 0.63075721263885498046875;
pub const HEX_D0: f32 = DEC_208;
pub const DEC_209: f32 = 0.63759696483612060546875;
pub const HEX_D1: f32 = DEC_209;
pub const DEC_210: f32 = 0.6444797515869140625;
pub const HEX_D2: f32 = DEC_210;
pub const DEC_211: f32 = 0.65140569210052490234375;
pub const HEX_D3: f32 = DEC_211;
pub const DEC_212: f32 = 0.658374845981597900390625;
pub const HEX_D4: f32 = DEC_212;
pub const DEC_213: f32 = 0.665387332439422607421875;
pub const HEX_D5: f32 = DEC_213;
pub const DEC_214: f32 = 0.672443211078643798828125;
pub const HEX_D6: f32 = DEC_214;
pub const DEC_215: f32 = 0.67954254150390625;
pub const HEX_D7: f32 = DEC_215;
pub const DEC_216: f32 = 0.68668544292449951171875;
pub const HEX_D8: f32 = DEC_216;
pub const DEC_218: f32 = 0.7011020183563232421875;
pub const HEX_DA: f32 = DEC_218;
pub const DEC_219: f32 = 0.7083759307861328125;
pub const HEX_DB: f32 = DEC_219;
pub const DEC_220: f32 = 0.715693652629852294921875;
pub const HEX_DC: f32 = DEC_220;
pub const DEC_221: f32 = 0.72305524349212646484375;
pub const HEX_DD: f32 = DEC_221;
pub const DEC_222: f32 = 0.7304608821868896484375;
pub const HEX_DE: f32 = DEC_222;
pub const DEC_224: f32 = 0.74540436267852783203125;
pub const HEX_E0: f32 = DEC_224;
pub const DEC_225: f32 = 0.7529423236846923828125;
pub const HEX_E1: f32 = DEC_225;
pub const DEC_226: f32 = 0.76052463054656982421875;
pub const HEX_E2: f32 = DEC_226;
pub const DEC_228: f32 = 0.775822341442108154296875;
pub const HEX_E4: f32 = DEC_228;
pub const DEC_230: f32 = 0.79129803180694580078125;
pub const HEX_E6: f32 = DEC_230;
pub const DEC_232: f32 = 0.80695235729217529296875;
pub const HEX_E8: f32 = DEC_232;
pub const DEC_233: f32 = 0.814846694469451904296875;
pub const HEX_E9: f32 = DEC_233;
pub const DEC_235: f32 = 0.830769956111907958984375;
pub const HEX_EB: f32 = DEC_235;
pub const DEC_237: f32 = 0.84687328338623046875;
pub const HEX_ED: f32 = DEC_237;
pub const DEC_238: f32 = 0.854992687702178955078125;
pub const HEX_EE: f32 = DEC_238;
pub const DEC_239: f32 = 0.8631572723388671875;
pub const HEX_EF: f32 = DEC_239;
pub const DEC_240: f32 = 0.8713672161102294921875;
pub const HEX_F0: f32 = DEC_240;
pub const DEC_244: f32 = 0.904661357402801513671875;
pub const HEX_F4: f32 = DEC_244;
pub const DEC_245: f32 = 0.91309869289398193359375;
pub const HEX_F5: f32 = DEC_245;
pub const DEC_248: f32 = 0.938685894012451171875;
pub const HEX_F8: f32 = DEC_248;
pub const DEC_250: f32 = 0.95597350597381591796875;
pub const HEX_FA: f32 = DEC_250;
pub const DEC_251: f32 = 0.96468627452850341796875;
pub const HEX_FB: f32 = DEC_251;
pub const DEC_252: f32 = 0.973445475101470947265625;
pub const HEX_FC: f32 = DEC_252;
pub const DEC_253: f32 = 0.98225057125091552734375;
pub const HEX_FD: f32 = DEC_253;
pub const DEC_255: f32 = 1.;
pub const HEX_FF: f32 = DEC_255;

/// #F0F8FF Alice Blue
pub const ALICE_BLUE: Color = Color::rgb_linear(HEX_F0, HEX_F8, HEX_FF);
/// #FAEBD7 Antique White
pub const ANTIQUE_WHITE: Color = Color::rgb_linear(HEX_FA, HEX_EB, HEX_D7);
/// #00FFFF Aqua
pub const AQUA: Color = Color::rgb_linear(HEX_00, HEX_FF, HEX_FF);
/// #7FFFD4 Aquamarine
pub const AQUAMARINE: Color = Color::rgb_linear(HEX_7F, HEX_FF, HEX_D4);
/// #F0FFFF Azure
pub const AZURE: Color = Color::rgb_linear(HEX_F0, HEX_FF, HEX_FF);
/// #F5F5DC Beige
pub const BEIGE: Color = Color::rgb_linear(HEX_F5, HEX_F5, HEX_DC);
/// #FFE4C4 Bisque
pub const BISQUE: Color = Color::rgb_linear(HEX_FF, HEX_E4, HEX_C4);
/// #000000 Black
pub const BLACK: Color = Color::rgb_linear(HEX_00, HEX_00, HEX_00);
/// #FFEBCD Blanched Almond
pub const BLANCHED_ALMOND: Color = Color::rgb_linear(HEX_FF, HEX_EB, HEX_CD);
/// #0000FF Blue
pub const BLUE: Color = Color::rgb_linear(HEX_00, HEX_00, HEX_FF);
/// #8A2BE2 Blue Violet
pub const BLUE_VIOLET: Color = Color::rgb_linear(HEX_8A, HEX_2B, HEX_E2);
/// #A52A2A Brown
pub const BROWN: Color = Color::rgb_linear(HEX_A5, HEX_2A, HEX_2A);
/// #DEB887 Burly Wood
pub const BURLY_WOOD: Color = Color::rgb_linear(HEX_DE, HEX_B8, HEX_87);
/// #5F9EA0 Cadet Blue
pub const CADET_BLUE: Color = Color::rgb_linear(HEX_5F, HEX_9E, HEX_A0);
/// #7FFF00 Chartreuse
pub const CHARTREUSE: Color = Color::rgb_linear(HEX_7F, HEX_FF, HEX_00);
/// #D2691E Chocolate
pub const CHOCOLATE: Color = Color::rgb_linear(HEX_D2, HEX_69, HEX_1E);
/// #FF7F50 Coral
pub const CORAL: Color = Color::rgb_linear(HEX_FF, HEX_7F, HEX_50);
/// #6495ED Cornflower Blue
pub const CORNFLOWER_BLUE: Color = Color::rgb_linear(HEX_64, HEX_95, HEX_ED);
/// #FFF8DC Cornsilk
pub const CORNSILK: Color = Color::rgb_linear(HEX_FF, HEX_F8, HEX_DC);
/// #DC143C Crimson
pub const CRIMSON: Color = Color::rgb_linear(HEX_DC, HEX_14, HEX_3C);
/// #00FFFF Cyan
pub const CYAN: Color = Color::rgb_linear(HEX_00, HEX_FF, HEX_FF);
/// #00008B Dark Blue
pub const DARK_BLUE: Color = Color::rgb_linear(HEX_00, HEX_00, HEX_8B);
/// #008B8B Dark Cyan
pub const DARK_CYAN: Color = Color::rgb_linear(HEX_00, HEX_8B, HEX_8B);
/// #B8860B Dark Golden Rod
pub const DARK_GOLDEN_ROD: Color = Color::rgb_linear(HEX_B8, HEX_86, HEX_0B);
/// #A9A9A9 Dark Gray
pub const DARK_GRAY: Color = Color::rgb_linear(HEX_A9, HEX_A9, HEX_A9);
/// #A9A9A9 Dark Grey
pub const DARK_GREY: Color = Color::rgb_linear(HEX_A9, HEX_A9, HEX_A9);
/// #006400 Dark Green
pub const DARK_GREEN: Color = Color::rgb_linear(HEX_00, HEX_64, HEX_00);
/// #BDB76B Dark Khaki
pub const DARK_KHAKI: Color = Color::rgb_linear(HEX_BD, HEX_B7, HEX_6B);
/// #8B008B Dark Magenta
pub const DARK_MAGENTA: Color = Color::rgb_linear(HEX_8B, HEX_00, HEX_8B);
/// #556B2F Dark Olive Green
pub const DARK_OLIVE_GREEN: Color = Color::rgb_linear(HEX_55, HEX_6B, HEX_2F);
/// #FF8C00 Dark Orange
pub const DARK_ORANGE: Color = Color::rgb_linear(HEX_FF, HEX_8C, HEX_00);
/// #9932CC Dark Orchid
pub const DARK_ORCHID: Color = Color::rgb_linear(HEX_99, HEX_32, HEX_CC);
/// #8B0000 Dark Red
pub const DARK_RED: Color = Color::rgb_linear(HEX_8B, HEX_00, HEX_00);
/// #E9967A Dark Salmon
pub const DARK_SALMON: Color = Color::rgb_linear(HEX_E9, HEX_96, HEX_7A);
/// #8FBC8F Dark Sea Green
pub const DARK_SEA_GREEN: Color = Color::rgb_linear(HEX_8F, HEX_BC, HEX_8F);
/// #483D8B Dark Slate Blue
pub const DARK_SLATE_BLUE: Color = Color::rgb_linear(HEX_48, HEX_3D, HEX_8B);
/// #2F4F4F Dark Slate Gray
pub const DARK_SLATE_GRAY: Color = Color::rgb_linear(HEX_2F, HEX_4F, HEX_4F);
/// #2F4F4F Dark Slate Grey
pub const DARK_SLATE_GREY: Color = Color::rgb_linear(HEX_2F, HEX_4F, HEX_4F);
/// #00CED1 Dark Turquoise
pub const DARK_TURQUOISE: Color = Color::rgb_linear(HEX_00, HEX_CE, HEX_D1);
/// #9400D3 Dark Violet
pub const DARK_VIOLET: Color = Color::rgb_linear(HEX_94, HEX_00, HEX_D3);
/// #FF1493 Deep Pink
pub const DEEP_PINK: Color = Color::rgb_linear(HEX_FF, HEX_14, HEX_93);
/// #00BFFF Deep Sky Blue
pub const DEEP_SKY_BLUE: Color = Color::rgb_linear(HEX_00, HEX_BF, HEX_FF);
/// #696969 Dim Gray
pub const DIM_GRAY: Color = Color::rgb_linear(HEX_69, HEX_69, HEX_69);
/// #696969 Dim Grey
pub const DIM_GREY: Color = Color::rgb_linear(HEX_69, HEX_69, HEX_69);
/// #1E90FF Dodger Blue
pub const DODGER_BLUE: Color = Color::rgb_linear(HEX_1E, HEX_90, HEX_FF);
/// #B22222 Fire Brick
pub const FIRE_BRICK: Color = Color::rgb_linear(HEX_B2, HEX_22, HEX_22);
/// #FFFAF0 Floral White
pub const FLORAL_WHITE: Color = Color::rgb_linear(HEX_FF, HEX_FA, HEX_F0);
/// #228B22 Forest Green
pub const FOREST_GREEN: Color = Color::rgb_linear(HEX_22, HEX_8B, HEX_22);
/// #FF00FF Fuchsia
pub const FUCHSIA: Color = Color::rgb_linear(HEX_FF, HEX_00, HEX_FF);
/// #DCDCDC Gainsboro
pub const GAINSBORO: Color = Color::rgb_linear(HEX_DC, HEX_DC, HEX_DC);
/// #F8F8FF Ghost White
pub const GHOST_WHITE: Color = Color::rgb_linear(HEX_F8, HEX_F8, HEX_FF);
/// #FFD700 Gold
pub const GOLD: Color = Color::rgb_linear(HEX_FF, HEX_D7, HEX_00);
/// #DAA520 Golden Rod
pub const GOLDEN_ROD: Color = Color::rgb_linear(HEX_DA, HEX_A5, HEX_20);
/// #808080 Gray
pub const GRAY: Color = Color::rgb_linear(HEX_80, HEX_80, HEX_80);
/// #808080 Grey
pub const GREY: Color = Color::rgb_linear(HEX_80, HEX_80, HEX_80);
/// #008000 Green
pub const GREEN: Color = Color::rgb_linear(HEX_00, HEX_80, HEX_00);
/// #ADFF2F Green Yellow
pub const GREEN_YELLOW: Color = Color::rgb_linear(HEX_AD, HEX_FF, HEX_2F);
/// #F0FFF0 Honey Dew
pub const HONEY_DEW: Color = Color::rgb_linear(HEX_F0, HEX_FF, HEX_F0);
/// #FF69B4 Hot Pink
pub const HOT_PINK: Color = Color::rgb_linear(HEX_FF, HEX_69, HEX_B4);
/// #CD5C5C Indian Red
pub const INDIAN_RED: Color = Color::rgb_linear(HEX_CD, HEX_5C, HEX_5C);
/// #4B0082 Indigo
pub const INDIGO: Color = Color::rgb_linear(HEX_4B, HEX_00, HEX_82);
/// #FFFFF0 Ivory
pub const IVORY: Color = Color::rgb_linear(HEX_FF, HEX_FF, HEX_F0);
/// #F0E68C Khaki
pub const KHAKI: Color = Color::rgb_linear(HEX_F0, HEX_E6, HEX_8C);
/// #E6E6FA Lavender
pub const LAVENDER: Color = Color::rgb_linear(HEX_E6, HEX_E6, HEX_FA);
/// #FFF0F5 Lavender Blush
pub const LAVENDER_BLUSH: Color = Color::rgb_linear(HEX_FF, HEX_F0, HEX_F5);
/// #7CFC00 Lawn Green
pub const LAWN_GREEN: Color = Color::rgb_linear(HEX_7C, HEX_FC, HEX_00);
/// #FFFACD Lemon Chiffon
pub const LEMON_CHIFFON: Color = Color::rgb_linear(HEX_FF, HEX_FA, HEX_CD);
/// #ADD8E6 Light Blue
pub const LIGHT_BLUE: Color = Color::rgb_linear(HEX_AD, HEX_D8, HEX_E6);
/// #F08080 Light Coral
pub const LIGHT_CORAL: Color = Color::rgb_linear(HEX_F0, HEX_80, HEX_80);
/// #E0FFFF Light Cyan
pub const LIGHT_CYAN: Color = Color::rgb_linear(HEX_E0, HEX_FF, HEX_FF);
/// #FAFAD2 Light Golden Rod Yellow
pub const LIGHT_GOLDEN_ROD_YELLOW: Color = Color::rgb_linear(HEX_FA, HEX_FA, HEX_D2);
/// #D3D3D3 Light Gray
pub const LIGHT_GRAY: Color = Color::rgb_linear(HEX_D3, HEX_D3, HEX_D3);
/// #D3D3D3 Light Grey
pub const LIGHT_GREY: Color = Color::rgb_linear(HEX_D3, HEX_D3, HEX_D3);
/// #90EE90 Light Green
pub const LIGHT_GREEN: Color = Color::rgb_linear(HEX_90, HEX_EE, HEX_90);
/// #FFB6C1 Light Pink
pub const LIGHT_PINK: Color = Color::rgb_linear(HEX_FF, HEX_B6, HEX_C1);
/// #FFA07A Light Salmon
pub const LIGHT_SALMON: Color = Color::rgb_linear(HEX_FF, HEX_A0, HEX_7A);
/// #20B2AA Light Sea Green
pub const LIGHT_SEA_GREEN: Color = Color::rgb_linear(HEX_20, HEX_B2, HEX_AA);
/// #87CEFA Light Sky Blue
pub const LIGHT_SKY_BLUE: Color = Color::rgb_linear(HEX_87, HEX_CE, HEX_FA);
/// #778899 Light Slate Gray
pub const LIGHT_SLATE_GRAY: Color = Color::rgb_linear(HEX_77, HEX_88, HEX_99);
/// #778899 Light Slate Grey
pub const LIGHT_SLATE_GREY: Color = Color::rgb_linear(HEX_77, HEX_88, HEX_99);
/// #B0C4DE Light Steel Blue
pub const LIGHT_STEEL_BLUE: Color = Color::rgb_linear(HEX_B0, HEX_C4, HEX_DE);
/// #FFFFE0 Light Yellow
pub const LIGHT_YELLOW: Color = Color::rgb_linear(HEX_FF, HEX_FF, HEX_E0);
/// #00FF00 Lime
pub const LIME: Color = Color::rgb_linear(HEX_00, HEX_FF, HEX_00);
/// #32CD32 Lime Green
pub const LIME_GREEN: Color = Color::rgb_linear(HEX_32, HEX_CD, HEX_32);
/// #FAF0E6 Linen
pub const LINEN: Color = Color::rgb_linear(HEX_FA, HEX_F0, HEX_E6);
/// #FF00FF Magenta
pub const MAGENTA: Color = Color::rgb_linear(HEX_FF, HEX_00, HEX_FF);
/// #800000 Maroon
pub const MAROON: Color = Color::rgb_linear(HEX_80, HEX_00, HEX_00);
/// #66CDAA Medium Aqua Marine
pub const MEDIUM_AQUA_MARINE: Color = Color::rgb_linear(HEX_66, HEX_CD, HEX_AA);
/// #0000CD Medium Blue
pub const MEDIUM_BLUE: Color = Color::rgb_linear(HEX_00, HEX_00, HEX_CD);
/// #BA55D3 Medium Orchid
pub const MEDIUM_ORCHID: Color = Color::rgb_linear(HEX_BA, HEX_55, HEX_D3);
/// #9370DB Medium Purple
pub const MEDIUM_PURPLE: Color = Color::rgb_linear(HEX_93, HEX_70, HEX_DB);
/// #3CB371 Medium Sea Green
pub const MEDIUM_SEA_GREEN: Color = Color::rgb_linear(HEX_3C, HEX_B3, HEX_71);
/// #7B68EE Medium Slate Blue
pub const MEDIUM_SLATE_BLUE: Color = Color::rgb_linear(HEX_7B, HEX_68, HEX_EE);
/// #00FA9A Medium Spring Green
pub const MEDIUM_SPRING_GREEN: Color = Color::rgb_linear(HEX_00, HEX_FA, HEX_9A);
/// #48D1CC Medium Turquoise
pub const MEDIUM_TURQUOISE: Color = Color::rgb_linear(HEX_48, HEX_D1, HEX_CC);
/// #C71585 Medium Violet Red
pub const MEDIUM_VIOLET_RED: Color = Color::rgb_linear(HEX_C7, HEX_15, HEX_85);
/// #191970 Midnight Blue
pub const MIDNIGHT_BLUE: Color = Color::rgb_linear(HEX_19, HEX_19, HEX_70);
/// #F5FFFA Mint Cream
pub const MINT_CREAM: Color = Color::rgb_linear(HEX_F5, HEX_FF, HEX_FA);
/// #FFE4E1 Misty Rose
pub const MISTY_ROSE: Color = Color::rgb_linear(HEX_FF, HEX_E4, HEX_E1);
/// #FFE4B5 Moccasin
pub const MOCCASIN: Color = Color::rgb_linear(HEX_FF, HEX_E4, HEX_B5);
/// #FFDEAD Navajo White
pub const NAVAJO_WHITE: Color = Color::rgb_linear(HEX_FF, HEX_DE, HEX_AD);
/// #000080 Navy
pub const NAVY: Color = Color::rgb_linear(HEX_00, HEX_00, HEX_80);
/// #FDF5E6 Old Lace
pub const OLD_LACE: Color = Color::rgb_linear(HEX_FD, HEX_F5, HEX_E6);
/// #808000 Olive
pub const OLIVE: Color = Color::rgb_linear(HEX_80, HEX_80, HEX_00);
/// #6B8E23 Olive Drab
pub const OLIVE_DRAB: Color = Color::rgb_linear(HEX_6B, HEX_8E, HEX_23);
/// #FFA500 Orange
pub const ORANGE: Color = Color::rgb_linear(HEX_FF, HEX_A5, HEX_00);
/// #FF4500 Orange Red
pub const ORANGE_RED: Color = Color::rgb_linear(HEX_FF, HEX_45, HEX_00);
/// #DA70D6 Orchid
pub const ORCHID: Color = Color::rgb_linear(HEX_DA, HEX_70, HEX_D6);
/// #EEE8AA Pale Golden Rod
pub const PALE_GOLDEN_ROD: Color = Color::rgb_linear(HEX_EE, HEX_E8, HEX_AA);
/// #98FB98 Pale Green
pub const PALE_GREEN: Color = Color::rgb_linear(HEX_98, HEX_FB, HEX_98);
/// #AFEEEE Pale Turquoise
pub const PALE_TURQUOISE: Color = Color::rgb_linear(HEX_AF, HEX_EE, HEX_EE);
/// #DB7093 Pale Violet Red
pub const PALE_VIOLET_RED: Color = Color::rgb_linear(HEX_DB, HEX_70, HEX_93);
/// #FFEFD5 Papaya Whip
pub const PAPAYA_WHIP: Color = Color::rgb_linear(HEX_FF, HEX_EF, HEX_D5);
/// #FFDAB9 Peach Puff
pub const PEACH_PUFF: Color = Color::rgb_linear(HEX_FF, HEX_DA, HEX_B9);
/// #CD853F Peru
pub const PERU: Color = Color::rgb_linear(HEX_CD, HEX_85, HEX_3F);
/// #FFC0CB Pink
pub const PINK: Color = Color::rgb_linear(HEX_FF, HEX_C0, HEX_CB);
/// #DDA0DD Plum
pub const PLUM: Color = Color::rgb_linear(HEX_DD, HEX_A0, HEX_DD);
/// #B0E0E6 Powder Blue
pub const POWDER_BLUE: Color = Color::rgb_linear(HEX_B0, HEX_E0, HEX_E6);
/// #800080 Purple
pub const PURPLE: Color = Color::rgb_linear(HEX_80, HEX_00, HEX_80);
/// #663399 Rebecca Purple
pub const REBECCA_PURPLE: Color = Color::rgb_linear(HEX_66, HEX_33, HEX_99);
/// #FF0000 Red
pub const RED: Color = Color::rgb_linear(HEX_FF, HEX_00, HEX_00);
/// #BC8F8F Rosy Brown
pub const ROSY_BROWN: Color = Color::rgb_linear(HEX_BC, HEX_8F, HEX_8F);
/// #4169E1 Royal Blue
pub const ROYAL_BLUE: Color = Color::rgb_linear(HEX_41, HEX_69, HEX_E1);
/// #8B4513 Saddle Brown
pub const SADDLE_BROWN: Color = Color::rgb_linear(HEX_8B, HEX_45, HEX_13);
/// #FA8072 Salmon
pub const SALMON: Color = Color::rgb_linear(HEX_FA, HEX_80, HEX_72);
/// #F4A460 Sandy Brown
pub const SANDY_BROWN: Color = Color::rgb_linear(HEX_F4, HEX_A4, HEX_60);
/// #2E8B57 Sea Green
pub const SEA_GREEN: Color = Color::rgb_linear(HEX_2E, HEX_8B, HEX_57);
/// #FFF5EE Sea Shell
pub const SEA_SHELL: Color = Color::rgb_linear(HEX_FF, HEX_F5, HEX_EE);
/// #A0522D Sienna
pub const SIENNA: Color = Color::rgb_linear(HEX_A0, HEX_52, HEX_2D);
/// #C0C0C0 Silver
pub const SILVER: Color = Color::rgb_linear(HEX_C0, HEX_C0, HEX_C0);
/// #87CEEB Sky Blue
pub const SKY_BLUE: Color = Color::rgb_linear(HEX_87, HEX_CE, HEX_EB);
/// #6A5ACD Slate Blue
pub const SLATE_BLUE: Color = Color::rgb_linear(HEX_6A, HEX_5A, HEX_CD);
/// #708090 Slate Gray
pub const SLATE_GRAY: Color = Color::rgb_linear(HEX_70, HEX_80, HEX_90);
/// #708090 Slate Grey
pub const SLATE_GREY: Color = Color::rgb_linear(HEX_70, HEX_80, HEX_90);
/// #FFFAFA Snow
pub const SNOW: Color = Color::rgb_linear(HEX_FF, HEX_FA, HEX_FA);
/// #00FF7F Spring Green
pub const SPRING_GREEN: Color = Color::rgb_linear(HEX_00, HEX_FF, HEX_7F);
/// #4682B4 Steel Blue
pub const STEEL_BLUE: Color = Color::rgb_linear(HEX_46, HEX_82, HEX_B4);
/// #D2B48C Tan
pub const TAN: Color = Color::rgb_linear(HEX_D2, HEX_B4, HEX_8C);
/// #008080 Teal
pub const TEAL: Color = Color::rgb_linear(HEX_00, HEX_80, HEX_80);
/// #D8BFD8 Thistle
pub const THISTLE: Color = Color::rgb_linear(HEX_D8, HEX_BF, HEX_D8);
/// #FF6347 Tomato
pub const TOMATO: Color = Color::rgb_linear(HEX_FF, HEX_63, HEX_47);
/// #40E0D0 Turquoise
pub const TURQUOISE: Color = Color::rgb_linear(HEX_40, HEX_E0, HEX_D0);
/// #EE82EE Violet
pub const VIOLET: Color = Color::rgb_linear(HEX_EE, HEX_82, HEX_EE);
/// #F5DEB3 Wheat
pub const WHEAT: Color = Color::rgb_linear(HEX_F5, HEX_DE, HEX_B3);
/// #FFFFFF White
pub const WHITE: Color = Color::rgb_linear(HEX_FF, HEX_FF, HEX_FF);
/// #F5F5F5 White Smoke
pub const WHITE_SMOKE: Color = Color::rgb_linear(HEX_F5, HEX_F5, HEX_F5);
/// #FFFF00 Yellow
pub const YELLOW: Color = Color::rgb_linear(HEX_FF, HEX_FF, HEX_00);
/// #9ACD32 Yellow Green
pub const YELLOW_GREEN: Color = Color::rgb_linear(HEX_9A, HEX_CD, HEX_32);

struct ColorIterator {
    index: usize,
}

impl ColorIterator {
    fn new() -> Self {
        Self { index: 0 }
    }
}

impl Iterator for ColorIterator {
    type Item = Color;

    fn next(&mut self) -> Option<Self::Item> {
        let color = match self.index {
            0 => ALICE_BLUE,
            1 => ANTIQUE_WHITE,
            2 => AQUA,
            3 => AQUAMARINE,
            4 => AZURE,
            5 => BEIGE,
            6 => BISQUE,
            7 => BLACK,
            8 => BLANCHED_ALMOND,
            9 => BLUE,
            10 => BLUE_VIOLET,
            11 => BROWN,
            12 => BURLY_WOOD,
            13 => CADET_BLUE,
            14 => CHARTREUSE,
            15 => CHOCOLATE,
            16 => CORAL,
            17 => CORNFLOWER_BLUE,
            18 => CORNSILK,
            19 => CRIMSON,
            20 => CYAN,
            21 => DARK_BLUE,
            22 => DARK_CYAN,
            23 => DARK_GOLDEN_ROD,
            24 => DARK_GRAY,
            25 => DARK_GREY,
            26 => DARK_GREEN,
            27 => DARK_KHAKI,
            28 => DARK_MAGENTA,
            29 => DARK_OLIVE_GREEN,
            30 => DARK_ORANGE,
            31 => DARK_ORCHID,
            32 => DARK_RED,
            33 => DARK_SALMON,
            34 => DARK_SEA_GREEN,
            35 => DARK_SLATE_BLUE,
            36 => DARK_SLATE_GRAY,
            37 => DARK_SLATE_GREY,
            38 => DARK_TURQUOISE,
            39 => DARK_VIOLET,
            40 => DEEP_PINK,
            41 => DEEP_SKY_BLUE,
            42 => DIM_GRAY,
            43 => DIM_GREY,
            44 => DODGER_BLUE,
            45 => FIRE_BRICK,
            46 => FLORAL_WHITE,
            47 => FOREST_GREEN,
            48 => FUCHSIA,
            49 => GAINSBORO,
            50 => GHOST_WHITE,
            51 => GOLD,
            52 => GOLDEN_ROD,
            53 => GRAY,
            54 => GREY,
            55 => GREEN,
            56 => GREEN_YELLOW,
            57 => HONEY_DEW,
            58 => HOT_PINK,
            59 => INDIAN_RED,
            60 => INDIGO,
            61 => IVORY,
            62 => KHAKI,
            63 => LAVENDER,
            64 => LAVENDER_BLUSH,
            65 => LAWN_GREEN,
            66 => LEMON_CHIFFON,
            67 => LIGHT_BLUE,
            68 => LIGHT_CORAL,
            69 => LIGHT_CYAN,
            70 => LIGHT_GOLDEN_ROD_YELLOW,
            71 => LIGHT_GRAY,
            72 => LIGHT_GREY,
            73 => LIGHT_GREEN,
            74 => LIGHT_PINK,
            75 => LIGHT_SALMON,
            76 => LIGHT_SEA_GREEN,
            77 => LIGHT_SKY_BLUE,
            78 => LIGHT_SLATE_GRAY,
            79 => LIGHT_SLATE_GREY,
            80 => LIGHT_STEEL_BLUE,
            81 => LIGHT_YELLOW,
            82 => LIME,
            83 => LIME_GREEN,
            84 => LINEN,
            85 => MAGENTA,
            86 => MAROON,
            87 => MEDIUM_AQUA_MARINE,
            88 => MEDIUM_BLUE,
            89 => MEDIUM_ORCHID,
            90 => MEDIUM_PURPLE,
            91 => MEDIUM_SEA_GREEN,
            92 => MEDIUM_SLATE_BLUE,
            93 => MEDIUM_SPRING_GREEN,
            94 => MEDIUM_TURQUOISE,
            95 => MEDIUM_VIOLET_RED,
            96 => MIDNIGHT_BLUE,
            97 => MINT_CREAM,
            98 => MISTY_ROSE,
            99 => MOCCASIN,
            100 => NAVAJO_WHITE,
            101 => NAVY,
            102 => OLD_LACE,
            103 => OLIVE,
            104 => OLIVE_DRAB,
            105 => ORANGE,
            106 => ORANGE_RED,
            107 => ORCHID,
            108 => PALE_GOLDEN_ROD,
            109 => PALE_GREEN,
            110 => PALE_TURQUOISE,
            111 => PALE_VIOLET_RED,
            112 => PAPAYA_WHIP,
            113 => PEACH_PUFF,
            114 => PERU,
            115 => PINK,
            116 => PLUM,
            117 => POWDER_BLUE,
            118 => PURPLE,
            119 => REBECCA_PURPLE,
            120 => RED,
            121 => ROSY_BROWN,
            122 => ROYAL_BLUE,
            123 => SADDLE_BROWN,
            124 => SALMON,
            125 => SANDY_BROWN,
            126 => SEA_GREEN,
            127 => SEA_SHELL,
            128 => SIENNA,
            129 => SILVER,
            130 => SKY_BLUE,
            131 => SLATE_BLUE,
            132 => SLATE_GRAY,
            133 => SLATE_GREY,
            134 => SNOW,
            135 => SPRING_GREEN,
            136 => STEEL_BLUE,
            137 => TAN,
            138 => TEAL,
            139 => THISTLE,
            140 => TOMATO,
            141 => TURQUOISE,
            142 => VIOLET,
            143 => WHEAT,
            144 => WHITE,
            145 => WHITE_SMOKE,
            146 => YELLOW,
            147 => YELLOW_GREEN,
            _ => unreachable!(),
        };
        self.index += 1;
        if self.index > 147 {
            None
        } else {
            Some(color)
        }
    }
}

#[test]
fn test_color_iter() {
    for color in ColorIterator::new().skip(5).take(1) {
        dbg!(color);
    }
}
