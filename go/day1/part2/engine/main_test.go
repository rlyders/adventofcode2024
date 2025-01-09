package engine

import (
	"testing"
)

func Test_GetSimilarityScoreOfListsText(t *testing.T) {
	type args struct {
		lists string
	}
	tests := []struct {
		name    string
		args    args
		want    SimilarityResults
		wantErr bool
	}{
		{name: "",
			args: args{lists: `3   4
4   3
2   5
1   3
3   9
3   3`},
			want: SimilarityResults{
				Similarity_scores: []SimilarityScore{
					{
						Location_a:       1,
						Found_in_list_b:  0,
						Similarity_score: 0,
					}, {
						Location_a:       2,
						Found_in_list_b:  0,
						Similarity_score: 0,
					}, {
						Location_a:       3,
						Found_in_list_b:  3,
						Similarity_score: 9,
					}, {
						Location_a:       3,
						Found_in_list_b:  3,
						Similarity_score: 9,
					}, {
						Location_a:       3,
						Found_in_list_b:  3,
						Similarity_score: 9,
					}, {
						Location_a:       4,
						Found_in_list_b:  1,
						Similarity_score: 4,
					},
				},
				Total_similarity_score: 31,
				Elapseds:               nil,
			},
			wantErr: false},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got, err := GetSimilarityScoreOfListsText(tt.args.lists)
			if (err != nil) != tt.wantErr {
				t.Errorf("GetSimilarityScoreOfListsText() error = %v, wantErr %v", err, tt.wantErr)
				return
			}
			if got.Total_similarity_score != tt.want.Total_similarity_score {
				t.Errorf("GetSimilarityScoreOfListsText() = %v, want %v", got, tt.want)
			}
		})
	}
}
